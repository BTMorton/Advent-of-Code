use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read, Result};

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Character(String),
    List(Vec<usize>),
    Opt(Vec<usize>, Vec<usize>),
}

macro_rules! character {
    ($s: expr) => {
        Rule::Character($s.to_string())
    };
}

#[cfg(test)]
mod day19_tests {
    use super::*;

    macro_rules! list {
        ($($s: expr), *) => {
            Rule::List(vec![$($s,)*])
        };
    }
    macro_rules! opt {
        ([$($a: expr),*], [$($b: expr),*]) => {
            Rule::Opt(vec![$($a,)*], vec![$($b,)*])
        };
    }

    static TEST_INPUTA: &str = "0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"
";

    static TEST2_INPUTA: &str = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"
";

    static TEST2_INPUTB: &str = "ababbb
bababa
abbbab
aaabbb
aaaabbb
";

    static TEST3_INPUT: &str = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
";

    fn test1_rules() -> Vec<Option<Rule>> {
        vec![
            Some(list![1, 2]),
            Some(character!("a")),
            Some(opt![[1, 3], [3, 1]]),
            Some(character!("b")),
        ]
    }

    fn test2_rules() -> Vec<Option<Rule>> {
        vec![
            Some(list![4, 1, 5]),
            Some(opt![[2, 3], [3, 2]]),
            Some(opt![[4, 4], [5, 5]]),
            Some(opt![[4, 5], [5, 4]]),
            Some(character!("a")),
            Some(character!("b")),
        ]
    }

    #[test]
    fn should_parse_rule_strings() {
        assert_eq!(test1_rules(), parse_rules(TEST_INPUTA));
        assert_eq!(test2_rules(), parse_rules(TEST2_INPUTA));
    }

    #[test]
    fn should_parse_a_rule_string() {
        assert_eq!(Some((0, list![4, 1, 5])), parse_rule_string("0: 4 1 5"));
        assert_eq!(
            Some((1, opt!([2, 3], [3, 2]))),
            parse_rule_string("1: 2 3 | 3 2")
        );
        assert_eq!(Some((4, character!("a"))), parse_rule_string("4: \"a\""));
    }

    #[test]
    fn should_generate_a_regular_expression_string_for_a_rule() {
        assert_eq!(Some("a".to_string()), get_rule_regex(&test1_rules(), 1));
        assert_eq!(Some("b".to_string()), get_rule_regex(&test1_rules(), 3));
        assert_eq!(
            Some("(ab|ba)".to_string()),
            get_rule_regex(&test1_rules(), 2)
        );
        assert_eq!(
            Some("a(ab|ba)".to_string()),
            get_rule_regex(&test1_rules(), 0)
        );
        assert_eq!(
            Some("a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b".to_string()),
            get_rule_regex(&test2_rules(), 0)
        );
    }

    #[test]
    fn day19a_test() {
        assert_eq!(2, day19a(&test2_rules(), &TEST2_INPUTB.lines().collect()))
    }

    #[test]
    fn day19b_test() {
        let (rules, messages) = parse_input(TEST3_INPUT).unwrap();
        assert_eq!(12, day19b(&rules, &messages));
    }
}

fn generate_rule_list_regex(
    rules: &Vec<Option<Rule>>,
    list: &Vec<usize>,
    seen_rules: &mut HashMap<usize, String>,
) -> Option<String> {
    list.iter()
        .map(|&i| generate_regex(rules, i, seen_rules))
        .fold(Some("".to_string()), |acc, rule| Some(acc? + &rule?))
}

fn generate_regex(
    rules: &Vec<Option<Rule>>,
    index: usize,
    seen_rules: &mut HashMap<usize, String>,
) -> Option<String> {
    if seen_rules.contains_key(&index) {
        return Some(seen_rules[&index].clone());
    }

    let rule = match rules[index].as_ref()? {
        Rule::Character(c) => c.clone(),
        Rule::List(list) => generate_rule_list_regex(rules, list, seen_rules)?,
        Rule::Opt(a, b) => format!(
            "({}|{})",
            generate_rule_list_regex(rules, a, seen_rules)?,
            generate_rule_list_regex(rules, b, seen_rules)?
        )
        .to_string(),
    };

    seen_rules.insert(index, rule.clone());
    Some(rule)
}

fn get_rule_regex(rules: &Vec<Option<Rule>>, index: usize) -> Option<String> {
    let mut seen_rules = HashMap::<usize, String>::new();
    generate_regex(rules, index, &mut seen_rules)
}

fn parse_rules(input: &str) -> Vec<Option<Rule>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut output = Vec::<Option<Rule>>::new();

    for line in lines {
        match parse_rule_string(line) {
            Some((index, rule)) => {
                output.resize(max(index + 1, output.len()), None);
                output[index] = Some(rule);
            }
            _ => {}
        }
    }

    output
}

fn parse_number(number: &str) -> Option<usize> {
    number.trim().parse::<usize>().ok()
}

fn parse_rule_numbers(input: &str) -> Option<Vec<usize>> {
    let numbers = input.trim().split(" ").map(|n| parse_number(n));
    if numbers.clone().any(|n| n.is_none()) {
        None
    } else {
        Some(numbers.filter_map(|n| n).collect())
    }
}

fn parse_rule_content(input: &str) -> Option<Rule> {
    let rule = if input.starts_with("\"") {
        character!(input.trim().replace("\"", ""))
    } else if input.contains("|") {
        let options: Vec<Option<Vec<usize>>> = input
            .split("|")
            .map(|opt| parse_rule_numbers(opt))
            .collect();
        Rule::Opt(options[0].clone()?, options[1].clone()?)
    } else {
        Rule::List(parse_rule_numbers(input)?)
    };

    Some(rule)
}

fn parse_rule_string(input: &str) -> Option<(usize, Rule)> {
    let mut parts = input.split(": ");
    let rule_no = parse_number(parts.next()?)?;
    let rule_content = parse_rule_content(parts.next()?)?;

    Some((rule_no, rule_content))
}

fn parse_input(input: &str) -> Option<(Vec<Option<Rule>>, Vec<&str>)> {
    let mut parts = input.split("\n\n");
    let rules = parts.next()?;
    let messages = parts.next()?.lines().collect();

    Some((parse_rules(rules), messages))
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day19a(rules: &Vec<Option<Rule>>, messages: &Vec<&str>) -> usize {
    let regex_str = get_rule_regex(rules, 0).unwrap();
    let regex = Regex::new(format!("^{}$", regex_str).as_str()).unwrap();

    messages.iter().filter(|m| regex.is_match(m)).count()
}

fn day19b(rules: &Vec<Option<Rule>>, messages: &Vec<&str>) -> usize {
    //  This is using the knowledge that 0: 8 11
    //  and 8: 42 | 42 8, 11: 42 31 | 42 11 31
    //  means 8: (42)+ and 11: (42){x}(31){x}
    let rule42 = get_rule_regex(rules, 42).unwrap();
    let rule31 = get_rule_regex(rules, 31).unwrap();
    println!("{}", rule42);
    println!("{}", rule31);
    let regex = Regex::new(
        format!(
            "^({})+({1}{{1}}{2}{{1}}|{1}{{2}}{2}{{2}}|{1}{{3}}{2}{{3}}|{1}{{4}}{2}{{4}})$",
            rule42, rule42, rule31
        )
        .as_str(),
    )
    .unwrap();

    messages.iter().filter(|m| regex.is_match(m)).count()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let (rules, messages) = match parse_input(&input) {
        Some(tuple) => tuple,
        _ => return Err(Error::new(ErrorKind::Other, "Unable to parse input")),
    };

    use std::time::Instant;
    let total = Instant::now();

    let result = day19a(&rules, &messages);
    println!(
        "Day 19A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day19b(&rules, &messages);
    println!(
        "Day 19B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
