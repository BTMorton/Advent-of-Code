use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

struct PasswordPolicy {
    first: usize,
    second: usize,
    character: char,
}

struct PasswordAndPolicy {
    password: String,
    policy: PasswordPolicy,
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    static TEST_INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    #[test]
    fn should_parse_policy_into_struct() {
        let policy = parse_policy("1-3 a").unwrap();
        assert_eq!(policy.first, 1);
        assert_eq!(policy.second, 3);
        assert_eq!(policy.character, 'a');
    }

    #[test]
    fn should_parse_a_line_into_password_and_policy() {
        let parsed_line = parse_line("1-3 a: abcde").unwrap();
        assert_eq!(parsed_line.policy.first, 1);
        assert_eq!(parsed_line.policy.second, 3);
        assert_eq!(parsed_line.policy.character, 'a');
        assert_eq!(parsed_line.password, "abcde");
    }

    #[test]
    fn should_return_true_if_password_matches_old_policy() {
        let test_policy = PasswordAndPolicy {
            policy: PasswordPolicy {
                first: 1,
                second: 3,
                character: 'a',
            },
            password: String::from("abcde"),
        };
        assert_eq!(true, does_password_meet_old_policy(&test_policy))
    }

    #[test]
    fn should_return_false_if_password_does_not_match_old_policy() {
        let test_policy = PasswordAndPolicy {
            policy: PasswordPolicy {
                first: 1,
                second: 3,
                character: 'b',
            },
            password: String::from("cdefg"),
        };
        assert_eq!(false, does_password_meet_old_policy(&test_policy))
    }

    #[test]
    fn should_return_true_if_password_matches_new_policy() {
        let test_policy = PasswordAndPolicy {
            policy: PasswordPolicy {
                first: 1,
                second: 3,
                character: 'a',
            },
            password: String::from("abcde"),
        };
        assert_eq!(true, does_password_meet_new_policy(&test_policy))
    }

    #[test]
    fn should_return_false_if_password_does_not_match_new_policy() {
        let test_policy = PasswordAndPolicy {
            policy: PasswordPolicy {
                first: 1,
                second: 3,
                character: 'b',
            },
            password: String::from("cdefg"),
        };
        assert_eq!(false, does_password_meet_new_policy(&test_policy))
    }

    #[test]
    fn day2a_tests() {
        assert_eq!(2, day2a(TEST_INPUT));
    }

    #[test]
    fn day2b_tests() {
        assert_eq!(1, day2b(TEST_INPUT));
    }
}

fn parse_policy(input: &str) -> Option<PasswordPolicy> {
    let parts: Vec<&str> = input.split(" ").collect();
    let range: Vec<&str> = parts[0].split("-").collect();

    Some(PasswordPolicy {
        first: range[0].parse::<usize>().unwrap_or(0),
        second: range[1].parse::<usize>().unwrap_or(0),
        character: parts[1].chars().nth(0)?,
    })
}

fn parse_line(input: &str) -> Option<PasswordAndPolicy> {
    let parts: Vec<&str> = input.split(":").collect();

    Some(PasswordAndPolicy {
        policy: parse_policy(parts[0].trim())?,
        password: String::from(parts[1].trim()),
    })
}

fn does_password_meet_old_policy(input: &PasswordAndPolicy) -> bool {
    let char_count = input
        .password
        .chars()
        .filter(|&c| c == input.policy.character)
        .count();
    return char_count >= input.policy.first && char_count <= input.policy.second;
}

fn does_password_meet_new_policy(input: &PasswordAndPolicy) -> bool {
    vec![
        input.password.chars().nth(input.policy.first - 1),
        input.password.chars().nth(input.policy.second - 1),
    ]
    .iter()
    .filter_map(|&x| x)
    .filter(|&c| c == input.policy.character)
    .count()
        == 1
}

fn day2a(input: &str) -> usize {
    input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| parse_line(line))
        .filter_map(|x| x)
        .filter(|line| does_password_meet_old_policy(line))
        .count()
}

fn day2b(input: &str) -> usize {
    input
        .split("\n")
        .filter(|&line| line != "")
        .map(|line| parse_line(line))
        .filter_map(|x| x)
        .filter(|line| does_password_meet_new_policy(line))
        .count()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day2a(input.as_str());
    println!("Day 2A - {}", result);

    let result = day2b(input.as_str());
    println!("Day 2B - {}", result);

    Ok(())
}
