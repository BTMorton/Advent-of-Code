use std::fs::File;
use std::io::{BufReader, Read, Result};

#[cfg(test)]
mod day13_tests {
    use super::*;

    #[test]
    fn should_find_time_until_next_bus() {
        assert_eq!(6, time_until_next(939, 7));
        assert_eq!(10, time_until_next(939, 13));
        assert_eq!(5, time_until_next(939, 59));
        assert_eq!(22, time_until_next(939, 31));
        assert_eq!(11, time_until_next(939, 19));
    }

    #[test]
    fn day12a_test() {
        let bus_ids = vec![
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19),
        ];
        assert_eq!(295, day12a(939, &bus_ids))
    }

    #[test]
    fn day12b_test() {
        let bus_ids = vec![
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19),
        ];
        assert_eq!(1068781, day12b(&bus_ids))
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn parse_input(input: &str) -> (i64, Vec<Option<i64>>) {
    let mut lines = input.lines();
    let current_time = lines.next().unwrap().parse::<i64>().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(",")
        .map(|bus_id| match bus_id {
            "x" => None,
            n => Some(n.parse::<i64>().unwrap()),
        })
        .collect();
    (current_time, bus_ids)
}

fn time_until_next(current_time: i64, bus_id: i64) -> i64 {
    bus_id - (current_time % bus_id)
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day12a(current_time: i64, bus_ids: &Vec<Option<i64>>) -> i64 {
    let result = bus_ids
        .iter()
        .filter_map(|&x| x)
        .map(|bus_id| (bus_id, time_until_next(current_time, bus_id)))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    result.0 * result.1
}

fn day12b(bus_ids: &Vec<Option<i64>>) -> i64 {
    let prod = bus_ids.iter().filter_map(|&x| x).product::<i64>();

    let sum: i64 = bus_ids
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, &opt)| {
            let bus_id = opt.unwrap();
            let p = prod / bus_id;
            let a = i as i64 % bus_id;
            a * mod_inv(p, bus_id).unwrap() * p
        })
        .sum();
    (sum % prod) - (bus_ids.len() as i64 - 1)
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let (current_time, bus_ids) = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day12a(current_time, &bus_ids);
    println!(
        "Day 12A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day12b(&bus_ids);
    println!(
        "Day 12B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
