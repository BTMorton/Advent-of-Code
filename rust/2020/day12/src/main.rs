use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::ops::{Add, Mul};

macro_rules! point {
    ($x: expr, $y: expr) => {
        Point { x: $x, y: $y }
    };
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    static TEST_INPUT: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn should_parse_a_direction() {
        assert_eq!(Action::MoveNorth(10), Action::from("N10"));
        assert_eq!(Action::Left(90), Action::from("L90"));
        assert_eq!(Action::Forward(5), Action::from("F5"));
        assert_eq!(Action::Right(270), Action::from("R270"));
    }

    #[test]
    fn should_add_two_points() {
        assert_eq!(point!(2, 3), point!(2, 0) + point!(0, 3));
        assert_eq!(point!(2, 3), point!(1, 1) + point!(1, 2));
        assert_eq!(point!(2, 3), point!(3, 5) + point!(-1, -2));
    }

    #[test]
    fn should_add_a_point_to_a_tuple() {
        assert_eq!(point!(2, 3), point!(2, 0) + (0, 3));
        assert_eq!(point!(2, 3), point!(1, 1) + (1, 2));
        assert_eq!(point!(2, 3), point!(3, 5) + (-1, -2));
    }

    #[test]
    fn should_calculate_manhattan_distance() {
        assert_eq!(2, point!(2, 0).get_manhattan());
        assert_eq!(2, point!(1, 1).get_manhattan());
        assert_eq!(5, point!(2, 3).get_manhattan());
        assert_eq!(8, point!(3, 5).get_manhattan());
    }

    #[test]
    fn should_multiply_a_direction() {
        assert_eq!(point!(10, 0), Direction::East * 10);
        assert_eq!(point!(0, 5), Direction::North * 5);
        assert_eq!(point!(-7, 0), Direction::West * 7);
        assert_eq!(point!(0, -3), Direction::South * 3);
    }

    #[test]
    fn should_turn_a_direction_left() {
        assert_eq!(Direction::North, Direction::East.turn_left());
        assert_eq!(Direction::West, Direction::North.turn_left());
        assert_eq!(Direction::South, Direction::West.turn_left());
        assert_eq!(Direction::East, Direction::South.turn_left());
    }

    #[test]
    fn should_turn_a_direction_right() {
        assert_eq!(Direction::South, Direction::East.turn_right());
        assert_eq!(Direction::East, Direction::North.turn_right());
        assert_eq!(Direction::North, Direction::West.turn_right());
        assert_eq!(Direction::West, Direction::South.turn_right());
    }

    #[test]
    fn should_apply_the_action_to_the_current_position() {
        assert_eq!(
            point!(0, 10),
            Action::MoveNorth(10).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(10, 0),
            Action::MoveEast(10).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(0, -10),
            Action::MoveSouth(10).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(-10, 0),
            Action::MoveWest(10).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(0, 0),
            Action::Left(90).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(0, 0),
            Action::Right(90).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(10, 0),
            Action::Forward(10).next_position(point!(0, 0), Direction::East)
        );
        assert_eq!(
            point!(0, 10),
            Action::Forward(10).next_position(point!(0, 0), Direction::North)
        );
        assert_eq!(
            point!(-10, 0),
            Action::Forward(10).next_position(point!(0, 0), Direction::West)
        );
        assert_eq!(
            point!(0, -10),
            Action::Forward(10).next_position(point!(0, 0), Direction::South)
        );
    }

    #[test]
    fn should_rotate_the_current_direction() {
        assert_eq!(
            Direction::East,
            Action::MoveNorth(10).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::East,
            Action::MoveEast(10).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::East,
            Action::MoveSouth(10).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::East,
            Action::MoveWest(10).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::East,
            Action::Forward(10).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::North,
            Action::Left(90).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::South,
            Action::Right(90).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::West,
            Action::Left(180).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::West,
            Action::Right(180).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::South,
            Action::Left(270).next_direction(Direction::East)
        );
        assert_eq!(
            Direction::North,
            Action::Right(270).next_direction(Direction::East)
        );
    }

    #[test]
    fn should_rotate_a_point_around_center() {
        assert_eq!(point!(0, 10), point!(10, 0).rotate_left());
        assert_eq!(point!(-10, 0), point!(0, 10).rotate_left());
        assert_eq!(point!(0, -10), point!(-10, 0).rotate_left());
        assert_eq!(point!(10, 0), point!(0, -10).rotate_left());
        assert_eq!(point!(-4, 10), point!(10, 4).rotate_left());

        assert_eq!(point!(0, -10), point!(10, 0).rotate_right());
        assert_eq!(point!(10, 0), point!(0, 10).rotate_right());
        assert_eq!(point!(0, 10), point!(-10, 0).rotate_right());
        assert_eq!(point!(-10, 0), point!(0, -10).rotate_right());
        assert_eq!(point!(4, -10), point!(10, 4).rotate_right());
    }

    #[test]
    fn day12a_test() {
        let actions = TEST_INPUT.lines().map(Action::from).collect();
        assert_eq!(25, day12a(&actions));
    }

    #[test]
    fn day12b_test() {
        let actions = TEST_INPUT.lines().map(Action::from).collect();
        assert_eq!(286, day12b(&actions));
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn get_manhattan(self) -> isize {
        self.x.abs() + self.y.abs()
    }

    fn rotate_left(self) -> Point {
        point!(-self.y, self.x)
    }

    fn rotate_right(self) -> Point {
        point!(self.y, -self.x)
    }
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<(isize, isize)> for Point {
    type Output = Self;
    fn add(self, other: (isize, isize)) -> Point {
        self + point!(other.0, other.1)
    }
}

impl Mul<isize> for Point {
    type Output = Point;
    fn mul(self, other: isize) -> Point {
        point!(self.x * other, self.y * other)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Mul<isize> for Direction {
    type Output = Point;
    fn mul(self, other: isize) -> Point {
        match self {
            Direction::North => point!(0, other),
            Direction::East => point!(other, 0),
            Direction::South => point!(0, -other),
            Direction::West => point!(-other, 0),
        }
    }
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Action {
    MoveNorth(isize),
    MoveSouth(isize),
    MoveEast(isize),
    MoveWest(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl From<&str> for Action {
    fn from(input: &str) -> Action {
        let (direction, number) = input.split_at(1);
        let distance = number.parse::<isize>().unwrap();

        match direction {
            "N" => Action::MoveNorth(distance),
            "S" => Action::MoveSouth(distance),
            "E" => Action::MoveEast(distance),
            "W" => Action::MoveWest(distance),
            "L" => Action::Left(distance),
            "R" => Action::Right(distance),
            "F" => Action::Forward(distance),
            _ => unimplemented!(),
        }
    }
}

impl Action {
    fn next_position(self, current_position: Point, current_direction: Direction) -> Point {
        match self {
            Action::MoveNorth(distance) => current_position + (0, distance),
            Action::MoveSouth(distance) => current_position + (0, -distance),
            Action::MoveEast(distance) => current_position + (distance, 0),
            Action::MoveWest(distance) => current_position + (-distance, 0),
            Action::Forward(distance) => current_position + (current_direction * distance),
            _ => current_position,
        }
    }

    fn next_direction(self, current_direction: Direction) -> Direction {
        match self {
            Action::Left(angle) => {
                (0..(angle / 90)).fold(current_direction, |dir, _| dir.turn_left())
            }
            Action::Right(angle) => {
                (0..(angle / 90)).fold(current_direction, |dir, _| dir.turn_right())
            }
            _ => current_direction,
        }
    }
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day12a(actions: &Vec<Action>) -> isize {
    let mut current_position = point!(0, 0);
    let mut current_direction = Direction::East;

    for action in actions {
        current_position = action.next_position(current_position, current_direction);
        current_direction = action.next_direction(current_direction);
    }

    current_position.get_manhattan()
}

fn day12b(actions: &Vec<Action>) -> isize {
    let mut current_position = point!(0, 0);
    let mut waypoint = point!(10, 1);

    for action in actions {
        match action {
            Action::Left(angle) => {
                waypoint = (0..(angle / 90)).fold(waypoint, |dir, _| dir.rotate_left())
            }
            Action::Right(angle) => {
                waypoint = (0..(angle / 90)).fold(waypoint, |dir, _| dir.rotate_right())
            }
            Action::Forward(units) => current_position = current_position + (waypoint * *units),
            _ => waypoint = action.next_position(waypoint, Direction::East),
        }
    }

    current_position.get_manhattan()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let actions = input.lines().map(Action::from).collect();

    use std::time::Instant;
    let total = Instant::now();

    let result = day12a(&actions);
    println!(
        "Day 12A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day12b(&actions);
    println!(
        "Day 12B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
