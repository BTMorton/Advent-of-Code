use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

type Point = (isize, isize, isize, isize);

#[cfg(test)]
mod day17_tests {
    use super::*;
    use std::iter::FromIterator;

    static TEST_INPUT: &str = ".#.
..#
###
";

    fn get_test_input() -> HashSet<Point> {
        HashSet::from_iter(vec![
            (1, 0, 0, 0),
            (2, 1, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
        ])
    }

    #[test]
    fn should_parse_input() {
        assert_eq!(get_test_input(), parse_input(TEST_INPUT));
    }

    #[test]
    fn should_calculate_neighbours_of_point() {
        let movements = Vec::from(NEIGHBOUR_MOVEMENTS);
        let neighbours = calculate_neighbours((0, 0, 0, 0), &movements);
        assert_eq!(26, neighbours.len());
        assert_eq!(movements, neighbours);
    }

    #[test]
    fn should_count_the_active_neighbours() {
        assert_eq!(
            1,
            count_active_neighbours(&Vec::from(NEIGHBOUR_MOVEMENTS), &get_test_input())
        );
        assert_eq!(
            3,
            count_active_neighbours(
                &NEIGHBOUR_MOVEMENTS
                    .iter()
                    .map(|&movement| add_point((1, 2, 0, 0), movement))
                    .collect(),
                &get_test_input()
            )
        );
    }

    #[test]
    fn should_mark_active_if_active_and_has_2_or_3_neighbours() {
        let input = get_test_input();
        let movement = Vec::from(NEIGHBOUR_MOVEMENTS);
        assert!(update_point((1, 2, 0, 0), &movement, &input));
        assert!(update_point((2, 2, 0, 0), &movement, &input));
        assert!(update_point((2, 1, 0, 0), &movement, &input));
    }

    #[test]
    fn should_mark_inactive_if_active_and_less_than_2_or_more_than_3_neighbours() {
        let input = get_test_input();
        let movement = Vec::from(NEIGHBOUR_MOVEMENTS);
        assert!(!update_point((1, 0, 0, 0), &movement, &input));
        assert!(!update_point((0, 2, 0, 0), &movement, &input));
    }

    #[test]
    fn should_mark_active_if_inactive_and_has_3_neighbours() {
        let input = get_test_input();
        let movement = Vec::from(NEIGHBOUR_MOVEMENTS);
        assert!(update_point((0, 1, 0, 0), &movement, &input));
        assert!(update_point((1, 3, 0, 0), &movement, &input));
    }

    #[test]
    fn should_mark_inactive_if_inactive_and_does_not_have_3_neighbours() {
        let input = get_test_input();
        let movement = Vec::from(NEIGHBOUR_MOVEMENTS);
        assert!(!update_point((0, 0, 0, 0), &movement, &input));
        assert!(!update_point((1, 1, 0, 0), &movement, &input));
    }

    #[test]
    fn day17a_test() {
        assert_eq!(112, day17a(&get_test_input()));
    }

    #[test]
    fn day17b_test() {
        assert_eq!(848, day17b(&get_test_input()));
    }
}

static NEIGHBOUR_MOVEMENTS: [Point; 26] = [
    (-1, -1, -1, 0),
    (-1, -1, 0, 0),
    (-1, -1, 1, 0),
    (-1, 0, -1, 0),
    (-1, 0, 0, 0),
    (-1, 0, 1, 0),
    (-1, 1, -1, 0),
    (-1, 1, 0, 0),
    (-1, 1, 1, 0),
    (0, -1, -1, 0),
    (0, -1, 0, 0),
    (0, -1, 1, 0),
    (0, 0, -1, 0),
    (0, 0, 1, 0),
    (0, 1, -1, 0),
    (0, 1, 0, 0),
    (0, 1, 1, 0),
    (1, -1, -1, 0),
    (1, -1, 0, 0),
    (1, -1, 1, 0),
    (1, 0, -1, 0),
    (1, 0, 0, 0),
    (1, 0, 1, 0),
    (1, 1, -1, 0),
    (1, 1, 0, 0),
    (1, 1, 1, 0),
];

fn update_point(
    point: Point,
    neighbour_options: &Vec<Point>,
    current_state: &HashSet<Point>,
) -> bool {
    let neighbours = calculate_neighbours(point, neighbour_options);
    let neighbour_count = count_active_neighbours(&neighbours, current_state);

    neighbour_count == 3 || (current_state.contains(&point) && neighbour_count == 2)
}

fn count_active_neighbours(neighbours: &Vec<Point>, current_state: &HashSet<Point>) -> usize {
    neighbours
        .iter()
        .filter(|point| current_state.contains(point))
        .count()
}

fn add_point(a: Point, b: Point) -> Point {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

fn calculate_neighbours(point: Point, movements: &Vec<Point>) -> Vec<Point> {
    movements
        .iter()
        .map(|&movement| add_point(point, movement))
        .collect()
}

fn parse_input(input: &str) -> HashSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as isize, y as isize, 0, 0))
        })
        .collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn run_program(initial_state: &HashSet<Point>, movements: &Vec<Point>) -> usize {
    let mut state = initial_state.clone();

    for _ in 0..6 {
        let to_check: HashSet<Point> = state
            .iter()
            .flat_map(|&p| {
                let mut v = calculate_neighbours(p, movements);
                v.push(p);
                v
            })
            .collect();

        state = to_check
            .iter()
            .filter(|&&point| update_point(point, movements, &state))
            .map(|&point| point)
            .collect();
    }

    state.len()
}

fn day17a(initial_state: &HashSet<Point>) -> usize {
    let movements = Vec::from(NEIGHBOUR_MOVEMENTS);
    run_program(initial_state, &movements)
}

fn day17b(initial_state: &HashSet<Point>) -> usize {
    let mut movements: Vec<Point> = (-1..=1)
        .flat_map(|w| {
            NEIGHBOUR_MOVEMENTS
                .iter()
                .map(move |&movement| add_point(movement, (0, 0, 0, w)))
        })
        .collect();

    movements.extend(vec![(0, 0, 0, -1), (0, 0, 0, 1)]);

    run_program(initial_state, &movements)
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let program = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day17a(&program);
    println!(
        "Day 17A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day17b(&program);
    println!(
        "Day 17B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
