use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[derive(Debug, PartialEq)]
struct TicketSpec {
    fields: HashMap<String, HashSet<usize>>,
    all_fields: HashSet<usize>,
}

#[derive(Debug, PartialEq)]
struct Input {
    spec: TicketSpec,
    your_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

#[cfg(test)]
mod day16_tests {
    use super::*;

    static TEST_INPUT: &str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    static TEST2_INPUT: &str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    fn get_test_input() -> Input {
        let fields: HashMap<String, HashSet<usize>> = vec![
            ("class", [(1..=3), (5..=7)]),
            ("row", [(6..=11), (33..=44)]),
            ("seat", [(13..=40), (45..=50)]),
        ]
        .iter()
        .map(|(field, ranges)| {
            (
                field.to_string(),
                ranges
                    .iter()
                    .flat_map(|it| it.clone().into_iter())
                    .collect(),
            )
        })
        .collect();
        Input {
            spec: TicketSpec {
                fields: fields.clone(),
                all_fields: fields.values().flat_map(|set| set).map(|&n| n).collect(),
            },
            your_ticket: vec![7, 1, 14],
            other_tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        }
    }

    fn get_test2_input() -> Input {
        let fields: HashMap<String, HashSet<usize>> = vec![
            ("class", [(0..=1), (4..=19)]),
            ("row", [(0..=5), (8..=19)]),
            ("seat", [(0..=13), (16..=19)]),
        ]
        .iter()
        .map(|(field, ranges)| {
            (
                field.to_string(),
                ranges
                    .iter()
                    .flat_map(|it| it.clone().into_iter())
                    .collect(),
            )
        })
        .collect();
        Input {
            spec: TicketSpec {
                fields: fields.clone(),
                all_fields: fields.values().flat_map(|set| set).map(|&n| n).collect(),
            },
            your_ticket: vec![11, 12, 13],
            other_tickets: vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]],
        }
    }

    #[test]
    fn should_parse_an_input() {
        assert_eq!(get_test_input(), parse_input(TEST_INPUT));
        assert_eq!(get_test2_input(), parse_input(TEST2_INPUT));
    }

    #[test]
    fn should_find_invalid_values() {
        let input = get_test_input();
        let all_ticket_fields = input
            .other_tickets
            .iter()
            .flat_map(|v| v)
            .map(|&n| n)
            .collect();
        assert_eq!(
            vec![4, 55, 12],
            find_invalid_values(&input.spec.all_fields, &all_ticket_fields)
        );
    }

    #[test]
    fn should_filter_invalid_tickets() {
        assert_eq!(vec![vec![7, 3, 47]], find_valid_tickets(&get_test_input()));

        let input2 = get_test2_input();
        assert_eq!(input2.other_tickets, find_valid_tickets(&input2));
    }

    #[test]
    fn should_find_matching_fields_for_ticket_index() {
        let input = get_test2_input();
        assert_eq!(
            vec!["row".to_string()],
            find_matching_fields(&input.spec.fields, &input.other_tickets, 0)
        );
        assert_eq!(
            vec!["class".to_string(), "row".to_string()],
            find_matching_fields(&input.spec.fields, &input.other_tickets, 1)
        );
        assert_eq!(
            vec!["class".to_string(), "row".to_string(), "seat".to_string()],
            find_matching_fields(&input.spec.fields, &input.other_tickets, 2)
        );
    }

    #[test]
    fn should_find_matching_indexes_for_fields() {
        let input = get_test2_input();
        assert_eq!(
            vec![0, 1, 2],
            find_matching_indexes(
                &input.spec.fields.get(&"row".to_string()).unwrap(),
                &input.other_tickets
            )
        );
        assert_eq!(
            vec![1, 2],
            find_matching_indexes(
                &input.spec.fields.get(&"class".to_string()).unwrap(),
                &input.other_tickets
            )
        );
        assert_eq!(
            vec![2],
            find_matching_indexes(
                &input.spec.fields.get(&"seat".to_string()).unwrap(),
                &input.other_tickets
            )
        );
    }

    #[test]
    fn should_find_field_order() {
        assert_eq!(
            vec!["row", "class", "seat"],
            find_field_order(&get_test2_input())
        );
    }

    #[test]
    fn day16a_test() {
        assert_eq!(71, day16a(&get_test_input()));
    }
}

fn find_field_order(input: &Input) -> Vec<&str> {
    let valid_tickets = find_valid_tickets(input);
    let mut field_order: Vec<Option<&str>> = (0..input.spec.fields.len()).map(|_| None).collect();
    let mut matching_fields: HashMap<&str, Vec<usize>> = input
        .spec
        .fields
        .iter()
        .map(|(field, values)| {
            (
                field.as_str(),
                find_matching_indexes(values, &valid_tickets),
            )
        })
        .collect();

    while matching_fields.len() > 0 {
        for (field, indexes) in matching_fields.clone() {
            let options: Vec<usize> = indexes
                .iter()
                .filter(|&n| field_order[*n].is_none())
                .map(|&n| n)
                .collect();

            if options.len() == 1 {
                field_order[options[0]] = Some(field);
                matching_fields.remove(field);
            } else if options.len() == 0 {
                unreachable!();
            }
        }
    }

    field_order.iter().filter_map(|&f| f).collect()
}

fn find_matching_fields(
    fields: &HashMap<String, HashSet<usize>>,
    valid_tickets: &Vec<Vec<usize>>,
    ticket_index: usize,
) -> Vec<String> {
    let values: Vec<usize> = valid_tickets.iter().map(|v| v[ticket_index]).collect();
    fields
        .iter()
        .filter(|(_, value_set)| !values.iter().any(|n| !value_set.contains(&n)))
        .map(|(field, _)| field.clone())
        .collect()
}

fn find_matching_indexes(values: &HashSet<usize>, valid_tickets: &Vec<Vec<usize>>) -> Vec<usize> {
    let ticket_len: usize = valid_tickets[0].len();

    (0..ticket_len)
        .filter(|&index| !valid_tickets.iter().any(|v| !values.contains(&v[index])))
        .collect()
}

fn find_valid_tickets(input: &Input) -> Vec<Vec<usize>> {
    input
        .other_tickets
        .iter()
        .filter(|&ticket| !ticket.iter().any(|n| !input.spec.all_fields.contains(n)))
        .map(|v| v.clone())
        .collect()
}

fn find_invalid_values(valid_fields: &HashSet<usize>, input_values: &Vec<usize>) -> Vec<usize> {
    input_values
        .iter()
        .filter(|n| !valid_fields.contains(n))
        .map(|&n| n)
        .collect()
}

fn parse_field(line: &str) -> (String, HashSet<usize>) {
    let mut parts = line.split(":");
    let field = parts.next().unwrap().trim().to_string();
    let options: HashSet<usize> = parts
        .next()
        .unwrap()
        .trim()
        .split("or")
        .map(|range| range.trim().split("-").map(|n| n.parse::<usize>().unwrap()))
        .flat_map(|mut range_iter| (range_iter.next().unwrap()..=range_iter.next().unwrap()))
        .collect();

    (field, options)
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Input {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let fields: HashMap<String, HashSet<usize>> =
        parts[0].lines().map(|line| parse_field(line)).collect();
    let all_fields: HashSet<usize> = fields
        .values()
        .flat_map(|field| field.iter().map(|&n| n))
        .collect();

    Input {
        spec: TicketSpec {
            fields: fields,
            all_fields: all_fields,
        },
        your_ticket: parse_ticket(parts[1].lines().nth(1).unwrap()),
        other_tickets: parts[2]
            .lines()
            .skip(1)
            .map(|line| parse_ticket(line))
            .collect(),
    }
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day16a(program: &Input) -> usize {
    let all_values = program
        .other_tickets
        .iter()
        .flat_map(|v| v)
        .map(|&n| n)
        .collect();
    let invalid_fields = find_invalid_values(&program.spec.all_fields, &all_values);
    invalid_fields.iter().sum()
}

fn day16b(program: &Input) -> usize {
    let field_order = find_field_order(program);
    field_order
        .iter()
        .enumerate()
        .filter(|(_, field)| field.contains("departure"))
        .map(|(index, _)| program.your_ticket[index])
        .product()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let program = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day16a(&program);
    println!(
        "Day 16A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day16b(&program);
    println!(
        "Day 16B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
