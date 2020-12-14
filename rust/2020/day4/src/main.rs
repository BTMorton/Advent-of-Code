use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

#[allow(dead_code)]
struct Passport {
    birth_year: Option<u16>,
    issue_year: Option<u16>,
    expiration_year: Option<u16>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<u32>,
}
struct StringPassport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_colour: Option<String>,
    eye_colour: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

#[cfg(test)]
mod day4_tests {
    use super::*;

    static TEST_INPUT: &str = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    static TEST2_INPUT: &str = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    static FIRST_LINE: &str =
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";

    #[test]
    fn should_return_true_if_a_string_passport_is_complete() {
        let passport = StringPassport {
            birth_year: Some(String::from("1937")),
            issue_year: Some(String::from("2017")),
            expiration_year: Some(String::from("2020")),
            height: Some(String::from("183cm")),
            hair_colour: Some(String::from("#fffffd")),
            eye_colour: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: Some(String::from("147")),
        };
        assert_eq!(true, is_string_passport_valid(&passport));
    }

    #[test]
    fn should_return_true_if_a_string_passport_is_only_missing_country_id() {
        let passport = StringPassport {
            birth_year: Some(String::from("1937")),
            issue_year: Some(String::from("2017")),
            expiration_year: Some(String::from("2020")),
            height: Some(String::from("183cm")),
            hair_colour: Some(String::from("#fffffd")),
            eye_colour: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: None,
        };
        assert_eq!(true, is_string_passport_valid(&passport));
    }

    #[test]
    fn should_return_false_if_a_string_passport_is_missing_properties_other_than_country_id() {
        let passport = StringPassport {
            birth_year: Some(String::from("1937")),
            issue_year: Some(String::from("2017")),
            expiration_year: Some(String::from("2020")),
            height: Some(String::from("183cm")),
            hair_colour: None,
            eye_colour: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: None,
        };
        assert_eq!(false, is_string_passport_valid(&passport));
    }

    #[test]
    fn should_return_true_if_a_passport_is_valid() {
        let passport = Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183cm")),
            hair_colour: Some(String::from("#fffffd")),
            eye_colour: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: Some(147),
        };
        assert_eq!(true, is_passport_valid(&passport));
    }

    #[test]
    fn should_return_true_if_a_passport_is_valid_but_missing_country_id() {
        let passport = Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183cm")),
            hair_colour: Some(String::from("#fffffd")),
            eye_colour: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: None,
        };
        assert_eq!(true, is_passport_valid(&passport));
    }

    #[test]
    fn should_return_false_if_a_passport_is_missing_properties() {
        let passport = Passport {
            birth_year: Some(1937),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: None,
            hair_colour: Some(String::from("#fffffd")),
            eye_colour: Some(String::from("gry")),
            passport_id: Some(String::from("860033327")),
            country_id: None,
        };
        assert_eq!(false, is_passport_valid(&passport));
    }

    #[test]
    fn should_return_false_if_a_passport_is_invalid() {
        let passport = Passport {
            birth_year: Some(100),
            issue_year: Some(2017),
            expiration_year: Some(2020),
            height: Some(String::from("183in")),
            hair_colour: Some(String::from("asdfg")),
            eye_colour: Some(String::from("general")),
            passport_id: Some(String::from("860033327")),
            country_id: None,
        };
        assert_eq!(false, is_passport_valid(&passport));
    }

    #[test]
    fn should_convert_a_line_input_into_a_property_map() {
        let map = line_to_property_map(String::from(FIRST_LINE));
        assert_eq!(true, map.contains_key("ecl"));
        assert_eq!(true, map.contains_key("pid"));
        assert_eq!(true, map.contains_key("eyr"));
        assert_eq!(true, map.contains_key("hcl"));
        assert_eq!(true, map.contains_key("byr"));
        assert_eq!(true, map.contains_key("iyr"));
        assert_eq!(true, map.contains_key("cid"));
        assert_eq!(true, map.contains_key("hgt"));
    }

    #[test]
    fn should_convert_a_file_to_lines() {
        let lines = file_to_lines(TEST_INPUT);
        assert_eq!(4, lines.len());
        assert_eq!(FIRST_LINE, lines[0]);
    }

    #[test]
    fn should_convert_a_property_map_to_a_passport() {
        let map: HashMap<String, String> = [
            (String::from("ecl"), String::from("gry")),
            (String::from("pid"), String::from("860033327")),
            (String::from("eyr"), String::from("2020")),
            (String::from("hcl"), String::from("#fffffd")),
            (String::from("byr"), String::from("1937")),
            (String::from("iyr"), String::from("2017")),
            (String::from("cid"), String::from("147")),
            (String::from("hgt"), String::from("183cm")),
        ]
        .iter()
        .cloned()
        .collect();
        let passport = map_to_passport(map);

        assert_eq!(Some(1937), passport.birth_year);
        assert_eq!(Some(2017), passport.issue_year);
        assert_eq!(Some(2020), passport.expiration_year);
        assert_eq!(Some(String::from("183cm")), passport.height);
        assert_eq!(Some(String::from("#fffffd")), passport.hair_colour);
        assert_eq!(Some(String::from("gry")), passport.eye_colour);
        assert_eq!(Some(String::from("860033327")), passport.passport_id);
        assert_eq!(Some(147), passport.country_id);
    }

    #[test]
    fn should_return_true_if_a_year_is_in_range() {
        assert_eq!(true, num_in_range(Some(1999), 1920, 2002));
        assert_eq!(true, num_in_range(Some(2017), 2010, 2020));
        assert_eq!(true, num_in_range(Some(2022), 2020, 2030));
    }

    #[test]
    fn should_return_false_if_a_year_is_not_in_range() {
        assert_eq!(false, num_in_range(Some(1900), 1920, 2002));
        assert_eq!(false, num_in_range(Some(1900), 2010, 2020));
        assert_eq!(false, num_in_range(Some(1900), 2020, 2030));
        assert_eq!(false, num_in_range(None, 1920, 2002));
        assert_eq!(false, num_in_range(None, 2010, 2020));
        assert_eq!(false, num_in_range(None, 2020, 2030));
    }

    #[test]
    fn should_return_true_if_a_height_in_cm_is_in_range() {
        assert_eq!(true, height_is_valid(&Some(String::from("190cm"))));
    }

    #[test]
    fn should_return_false_if_a_height_in_cm_is_not_in_range() {
        assert_eq!(false, height_is_valid(&Some(String::from("60cm"))));
    }

    #[test]
    fn should_return_true_if_a_height_in_inch_is_in_range() {
        assert_eq!(true, height_is_valid(&Some(String::from("60in"))));
    }

    #[test]
    fn should_return_false_if_a_height_in_inch_is_not_in_range() {
        assert_eq!(false, height_is_valid(&Some(String::from("190in"))));
    }

    #[test]
    fn should_return_false_if_a_height_is_not_in_a_valid_form() {
        assert_eq!(false, height_is_valid(&Some(String::from("190"))));
        assert_eq!(false, height_is_valid(&None));
    }

    #[test]
    fn should_return_true_if_a_hair_color_is_correct() {
        assert_eq!(true, hair_colour_is_valid(&Some(String::from("#123abc"))));
    }

    #[test]
    fn should_return_false_if_a_hair_color_is_not_correct() {
        assert_eq!(false, hair_colour_is_valid(&Some(String::from("#123abz"))));
        assert_eq!(false, hair_colour_is_valid(&Some(String::from("123abc"))));
    }

    #[test]
    fn should_return_true_if_a_eye_color_is_correct() {
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("amb"))));
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("blu"))));
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("brn"))));
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("gry"))));
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("grn"))));
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("hzl"))));
        assert_eq!(true, eye_colour_is_valid(&Some(String::from("oth"))));
    }

    #[test]
    fn should_return_false_if_a_eye_color_is_not_correct() {
        assert_eq!(false, eye_colour_is_valid(&Some(String::from("abc"))));
        assert_eq!(false, eye_colour_is_valid(&Some(String::from("fjk"))));
        assert_eq!(false, eye_colour_is_valid(&Some(String::from("amber"))));
        assert_eq!(false, eye_colour_is_valid(&Some(String::from("asdf"))));
        assert_eq!(false, eye_colour_is_valid(&None));
    }

    #[test]
    fn should_return_true_is_the_passport_id_is_the_right_length_and_numeric() {
        assert_eq!(true, passport_id_is_valid(&Some(String::from("000000001"))));
        assert_eq!(true, passport_id_is_valid(&Some(String::from("012533040"))));
        assert_eq!(true, passport_id_is_valid(&Some(String::from("021572410"))));
        assert_eq!(true, passport_id_is_valid(&Some(String::from("896056539"))));
    }

    #[test]
    fn should_return_false_if_the_passport_id_is_the_wrong_length() {
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("0123456789")))
        );
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("3556412378")))
        );
        assert_eq!(false, passport_id_is_valid(&Some(String::from("186"))));
        assert_eq!(false, passport_id_is_valid(&Some(String::from("355641"))));
        assert_eq!(false, passport_id_is_valid(&None));
    }

    #[test]
    fn should_return_false_if_the_passport_id_is_not_numeric() {
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("abcdefghi")))
        );
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("jklmnopqr")))
        );
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("stuvwxyz1")))
        );
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("12345678a")))
        );
        assert_eq!(false, passport_id_is_valid(&None));
    }

    #[test]
    fn should_return_false_if_the_passport_id_is_0() {
        assert_eq!(
            false,
            passport_id_is_valid(&Some(String::from("000000000")))
        );
        assert_eq!(false, passport_id_is_valid(&Some(String::from("0"))));
    }

    #[test]
    fn day4a_tests() {
        assert_eq!(2, day4a(TEST_INPUT))
    }

    #[test]
    fn day4b_tests() {
        assert_eq!(4, day4b(TEST2_INPUT))
    }
}

fn passport_id_is_valid(passport_id: &Option<String>) -> bool {
    match passport_id {
        Some(s) => {
            s.len() == 9
                && match s.parse::<u32>() {
                    Ok(n) => n > 0,
                    Err(_) => false,
                }
        }
        None => false,
    }
}

fn num_in_range(num: Option<u16>, min: u16, max: u16) -> bool {
    match num {
        Some(n) => n >= min && n <= max,
        None => false,
    }
}

fn height_is_valid(height: &Option<String>) -> bool {
    match height {
        Some(s) => {
            let (num, suffix) = s.split_at(s.len() - 2);
            match num.parse::<u16>() {
                Ok(n) => match suffix {
                    "cm" => num_in_range(Some(n), 150, 193),
                    "in" => num_in_range(Some(n), 59, 76),
                    _ => false,
                },
                Err(_) => false,
            }
        }
        None => false,
    }
}

fn hair_colour_is_valid(hair_colour: &Option<String>) -> bool {
    match hair_colour {
        Some(s) => {
            let filtered: String = s
                .chars()
                .enumerate()
                .filter(|&(i, c)| i > 0 || c == '#')
                .filter(|&(i, c)| i == 0 || ((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')))
                .map(|(_, c)| c)
                .collect();
            filtered.len() == 7
        }
        None => false,
    }
}

fn eye_colour_is_valid(eye_colour: &Option<String>) -> bool {
    let valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    match eye_colour {
        Some(s) => valid.contains(&s.as_str()),
        None => false,
    }
}

fn file_to_string_passport(file_input: &str) -> Vec<StringPassport> {
    file_to_lines(file_input)
        .iter()
        .map(|line| line_to_property_map(String::from(line)))
        .map(|map| map_to_string_passport(map))
        .collect()
}

fn file_to_passport(file_input: &str) -> Vec<Passport> {
    file_to_lines(file_input)
        .iter()
        .map(|line| line_to_property_map(String::from(line)))
        .map(|map| map_to_passport(map))
        .collect()
}

fn map_to_string_passport(map: HashMap<String, String>) -> StringPassport {
    StringPassport {
        birth_year: map.get("byr").and_then(|s| Some(String::from(s))),
        issue_year: map.get("iyr").and_then(|s| Some(String::from(s))),
        expiration_year: map.get("eyr").and_then(|s| Some(String::from(s))),
        height: map.get("hgt").and_then(|s| Some(String::from(s))),
        hair_colour: map.get("hcl").and_then(|s| Some(String::from(s))),
        eye_colour: map.get("ecl").and_then(|s| Some(String::from(s))),
        passport_id: map.get("pid").and_then(|s| Some(String::from(s))),
        country_id: map.get("cid").and_then(|s| Some(String::from(s))),
    }
}

fn string_passport_to_passport(input: StringPassport) -> Passport {
    Passport {
        birth_year: input.birth_year.and_then(|s| match s.parse::<u16>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }),
        issue_year: input.issue_year.and_then(|s| match s.parse::<u16>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }),
        expiration_year: input.expiration_year.and_then(|s| match s.parse::<u16>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }),
        height: input.height,
        hair_colour: input.hair_colour,
        eye_colour: input.eye_colour,
        passport_id: input.passport_id,
        country_id: input.country_id.and_then(|s| match s.parse::<u32>() {
            Ok(n) => Some(n),
            Err(_) => None,
        }),
    }
}

fn map_to_passport(map: HashMap<String, String>) -> Passport {
    string_passport_to_passport(map_to_string_passport(map))
}

fn file_to_lines(file_input: &str) -> Vec<String> {
    let lines: Vec<String> = file_input
        .trim()
        .split("\n\n")
        .map(|line| line.replace("\n", " "))
        .collect();
    println!("{}", lines.iter().count());
    lines
}

fn line_to_property_map(line: String) -> HashMap<String, String> {
    line.split(" ")
        .map(|pair| {
            let parts: Vec<&str> = pair.split(":").collect();
            (String::from(parts[0]), String::from(parts[1]))
        })
        .collect()
}

fn is_string_passport_valid(passport: &StringPassport) -> bool {
    passport.birth_year.is_some()
        && passport.issue_year.is_some()
        && passport.expiration_year.is_some()
        && passport.height.is_some()
        && passport.hair_colour.is_some()
        && passport.eye_colour.is_some()
        && passport.passport_id.is_some()
}

fn is_passport_valid(passport: &Passport) -> bool {
    num_in_range(passport.birth_year, 1920, 2002)
        && num_in_range(passport.issue_year, 2010, 2020)
        && num_in_range(passport.expiration_year, 2020, 2030)
        && height_is_valid(&passport.height)
        && hair_colour_is_valid(&passport.hair_colour)
        && eye_colour_is_valid(&passport.eye_colour)
        && passport_id_is_valid(&passport.passport_id)
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day4a(input: &str) -> usize {
    file_to_string_passport(input)
        .iter()
        .filter(|&passport| is_string_passport_valid(passport))
        .count()
}

fn day4b(input: &str) -> usize {
    file_to_passport(input)
        .iter()
        .filter(|&passport| is_passport_valid(passport))
        .count()
}

fn main() -> Result<()> {
    let input = read_file("input")?;

    let result = day4a(input.as_str());
    println!("Day 4A - {}", result);

    let result = day4b(input.as_str());
    println!("Day 4B - {}", result);

    Ok(())
}
