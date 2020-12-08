use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Result};

struct BagCount {
    count: u32,
    colour: String,
}

struct Bag {
    colour: String,
    holds: Vec<BagCount>,
}

#[cfg(test)]
mod day7_tests {
    use super::*;

    static TEST_INPUT: &str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    static TEST2_INPUT: &str = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn should_parse_string_into_bag_count() {
        let count = parse_bag_count("2 muted yellow bags").unwrap();
        assert_eq!(2, count.count);
        assert_eq!("muted yellow", count.colour);

        let count = parse_bag_count("3 faded blue bags").unwrap();
        assert_eq!(3, count.count);
        assert_eq!("faded blue", count.colour);

        let count = parse_bag_count("1 bright white bag").unwrap();
        assert_eq!(1, count.count);
        assert_eq!("bright white", count.colour);

        let count = parse_bag_count("4 pink bags").unwrap();
        assert_eq!(4, count.count);
        assert_eq!("pink", count.colour);
    }

    #[test]
    fn should_parse_string_into_bag_definition() {
        let definition =
            parse_bag_definition("light red bags contain 1 bright white bag, 2 muted yellow bags.")
                .unwrap();
        assert_eq!("light red", definition.colour);
        assert_eq!(2, definition.holds.len());

        let definition =
            parse_bag_definition("bright white bags contain 1 shiny gold bag.").unwrap();
        assert_eq!("bright white", definition.colour);
        assert_eq!(1, definition.holds.len());

        let definition = parse_bag_definition("faded blue bags contain no other bags.").unwrap();
        assert_eq!("faded blue", definition.colour);
        assert_eq!(0, definition.holds.len());
    }

    #[test]
    fn day7a_tests() {
        assert_eq!(4, day7a(TEST_INPUT));
    }

    #[test]
    fn day7b_tests() {
        assert_eq!(32, day7b(TEST_INPUT));
        assert_eq!(126, day7b(TEST2_INPUT));
    }
}

fn parse_bag_definition(input: &str) -> Option<Bag> {
    let parts: Vec<&str> = input.trim_end_matches(".").split("contain").collect();
    let colour = parts[0].trim();

    if !colour.ends_with("bags") {
        println!("Not a bag");
        return None;
    }

    let bags = parts[1].trim();
    let holds: Vec<BagCount> = match bags {
        "no other bags" => Vec::new(),
        _ => bags
            .split(",")
            .map(|part| parse_bag_count(part.trim()))
            .filter_map(|x| x)
            .collect(),
    };

    Some(Bag {
        colour: String::from(colour.trim_end_matches("bags").trim()),
        holds: holds,
    })
}

fn parse_bag_count(input: &str) -> Option<BagCount> {
    if !(input.ends_with("bags") || input.ends_with("bag")) {
        return None;
    }

    let parts = input.split(" ").collect::<Vec<&str>>();
    let (count_parts, colour_parts) = parts.split_at(1);
    let bag_index = colour_parts.len() - 1;

    match count_parts[0].parse::<u32>() {
        Ok(n) => Some(BagCount {
            count: n,
            colour: colour_parts[..bag_index].join(" "),
        }),
        Err(_) => None,
    }
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn can_hold_shiny_gold(
    colour: &String,
    map: &HashMap<String, Vec<BagCount>>,
    seen: &mut HashMap<String, bool>,
) -> bool {
    if seen.contains_key(colour) {
        return *seen.get(colour).unwrap();
    }

    seen.insert(colour.clone(), false);

    let can_hold = map[colour].iter().any(|held| {
        held.colour == String::from("shiny gold") || can_hold_shiny_gold(&held.colour, map, seen)
    });

    seen.insert(colour.clone(), can_hold);
    can_hold
}

fn create_bag_map(input: &str) -> HashMap<String, Vec<BagCount>> {
    input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| parse_bag_definition(line))
        .filter_map(|x| x)
        .map(|bag| (bag.colour, bag.holds))
        .collect()
}

fn count_required_bags_to_fill(
    colour: &String,
    map: &HashMap<String, Vec<BagCount>>,
    seen: &mut HashMap<String, u32>,
) -> u32 {
    if seen.contains_key(colour) {
        return *seen.get(colour).unwrap();
    }

    let required_count = map[colour]
        .iter()
        .map(|held| held.count * (1 + count_required_bags_to_fill(&held.colour, map, seen)))
        .sum();

    seen.insert(colour.clone(), required_count);
    required_count
}

fn day7a(input: &str) -> usize {
    let bag_map = create_bag_map(input);

    let mut seen = HashMap::<String, bool>::new();
    bag_map
        .iter()
        .filter(|(colour, _)| can_hold_shiny_gold(colour.clone(), &bag_map, &mut seen))
        .count()
}

fn day7b(input: &str) -> u32 {
    let bag_map = create_bag_map(input);

    let mut seen = HashMap::<String, u32>::new();
    count_required_bags_to_fill(&String::from("shiny gold"), &bag_map, &mut seen)
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day7a(input.as_str());
    println!("Day 7A - {}", result);

    let result = day7b(input.as_str());
    println!("Day 7B - {}", result);

    Ok(())
}
