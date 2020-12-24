use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Movement {
    East,
    NorthEast,
    SouthEast,
    West,
    NorthWest,
    SouthWest,
}

type Point = (isize, isize);

#[cfg(test)]
mod day24_test {
    use super::*;

    static TEST_INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
";

    fn get_test_input() -> Vec<Vec<Movement>> {
        parse_input(TEST_INPUT)
    }

    #[test]
    fn should_parse_a_tile_movement_line() {
        assert_eq!(
            vec![
                Movement::SouthEast,
                Movement::SouthEast,
                Movement::NorthWest,
                Movement::NorthEast,
                Movement::NorthEast,
                Movement::NorthEast,
                Movement::West,
                Movement::SouthEast,
                Movement::East,
                Movement::SouthWest,
                Movement::West,
                Movement::SouthWest,
                Movement::SouthWest,
                Movement::West,
                Movement::NorthEast,
                Movement::NorthEast,
                Movement::West,
                Movement::SouthEast,
                Movement::West,
                Movement::SouthWest,
            ],
            parse_line("sesenwnenenewseeswwswswwnenewsewsw")
        );
    }

    #[test]
    fn should_apply_a_movement_to_a_coordinate() {
        assert_eq!((-1, 0), apply_movement(Movement::West, (0, 0)));
        assert_eq!((0, -1), apply_movement(Movement::NorthWest, (0, 0)));
        assert_eq!((-1, 1), apply_movement(Movement::SouthWest, (0, 0)));
        assert_eq!((1, 0), apply_movement(Movement::East, (0, 0)));
        assert_eq!((1, -1), apply_movement(Movement::NorthEast, (0, 0)));
        assert_eq!((0, 1), apply_movement(Movement::SouthEast, (0, 0)));
    }

    #[test]
    fn should_find_a_coordinate_from_movement_list() {
        assert_eq!(
            (0, 0),
            apply_movements(
                &vec![
                    Movement::NorthWest,
                    Movement::West,
                    Movement::SouthWest,
                    Movement::East,
                    Movement::East
                ],
                (0, 0)
            )
        )
    }

    #[test]
    fn should_find_a_tiles_neighbours() {
        assert_eq!(
            vec![(-1, 0), (0, -1), (-1, 1), (1, 0), (1, -1), (0, 1),],
            find_neighbours((0, 0))
        );
    }

    #[test]
    fn should_flip_a_tile_if_it_is_not_flipped_and_has_2_flipped_neighbours() {
        assert!(update_tile(
            (0, 0),
            &vec![(1, 0), (0, 1)].into_iter().collect()
        ));
    }

    #[test]
    fn should_unflip_a_tile_if_it_is_flipped_and_has_no_flipped_neighbours() {
        assert!(!update_tile((0, 0), &vec![(0, 0)].into_iter().collect()));
    }

    #[test]
    fn should_unflip_a_tile_if_it_is_flipped_and_has_more_than_2_flipped_neighbours() {
        assert!(!update_tile(
            (0, 0),
            &vec![(0, 0), (0, 1), (1, 0), (-1, 0), (0, -1)]
                .into_iter()
                .collect()
        ));
    }

    #[test]
    fn should_not_flip_a_tile_if_it_is_not_flipped_and_does_not_have_2_flipped_neighbours() {
        assert!(!update_tile((0, 0), &vec![(1, 0)].into_iter().collect()));
        assert!(!update_tile(
            (0, 0),
            &vec![(1, 0), (0, 1), (-1, 0)].into_iter().collect()
        ));
    }

    #[test]
    fn should_not_unflip_a_tile_if_it_is_flipped_and_has_2_flipped_neighbours() {
        assert!(update_tile(
            (0, 0),
            &vec![(0, 0), (1, 0), (0, 1)].into_iter().collect()
        ));
    }

    #[test]
    fn should_not_unflip_a_tile_if_it_is_flipped_and_has_1_flipped_neighbour() {
        assert!(update_tile(
            (0, 0),
            &vec![(0, 0), (1, 0)].into_iter().collect()
        ));
    }

    #[test]
    fn should_update_the_tiles_each_day() {
        let input = get_flipped_tiles(&get_test_input());

        let result = update_tiles(&input);
        assert_eq!(15, result.len());

        let result = update_tiles(&result);
        assert_eq!(12, result.len());

        let result = update_tiles(&result);
        assert_eq!(25, result.len());
    }

    #[test]
    fn day24a_test() {
        assert_eq!(10, day24a(&get_test_input()));
    }

    #[test]
    fn day24b_test() {
        assert_eq!(2208, day24b(&get_test_input()));
    }
}

fn update_tiles(flipped_tiles: &HashSet<Point>) -> HashSet<Point> {
    flipped_tiles
        .iter()
        .flat_map(|&point| find_neighbours(point))
        .filter(|&point| update_tile(point, flipped_tiles))
        .collect()
}

fn update_tile(point: Point, flipped_tiles: &HashSet<Point>) -> bool {
    let flipped_neighbours = find_neighbours(point)
        .iter()
        .filter(|neighbour| flipped_tiles.contains(neighbour))
        .count();

    if flipped_tiles.contains(&point) {
        flipped_neighbours > 0 && flipped_neighbours <= 2
    } else {
        flipped_neighbours == 2
    }
}

fn find_neighbours(point: Point) -> Vec<Point> {
    vec![
        apply_movement(Movement::West, point),
        apply_movement(Movement::NorthWest, point),
        apply_movement(Movement::SouthWest, point),
        apply_movement(Movement::East, point),
        apply_movement(Movement::NorthEast, point),
        apply_movement(Movement::SouthEast, point),
    ]
}

fn apply_movement(movement: Movement, point: Point) -> Point {
    match movement {
        Movement::East => (point.0 + 1, point.1),
        Movement::NorthEast => (point.0 + 1, point.1 - 1),
        Movement::SouthEast => (point.0, point.1 + 1),
        Movement::West => (point.0 - 1, point.1),
        Movement::NorthWest => (point.0, point.1 - 1),
        Movement::SouthWest => (point.0 - 1, point.1 + 1),
    }
}

fn apply_movements(movements: &Vec<Movement>, point: Point) -> Point {
    movements
        .iter()
        .fold(point, |acc, &movement| apply_movement(movement, acc))
}

fn get_flipped_tiles(movement_lists: &Vec<Vec<Movement>>) -> HashSet<Point> {
    let mut flipped_tiles = HashSet::<Point>::new();

    for movements in movement_lists {
        let target = apply_movements(movements, (0, 0));

        if flipped_tiles.contains(&target) {
            flipped_tiles.remove(&target);
        } else {
            flipped_tiles.insert(target);
        }
    }

    flipped_tiles
}

fn parse_line(line: &str) -> Vec<Movement> {
    let chars: Vec<char> = line.chars().collect();
    let mut index = 0;
    let mut movements = Vec::<Movement>::new();

    while index < chars.len() {
        match chars[index] {
            'w' => movements.push(Movement::West),
            'e' => movements.push(Movement::East),
            's' => {
                index += 1;
                match chars[index] {
                    'e' => movements.push(Movement::SouthEast),
                    'w' => movements.push(Movement::SouthWest),
                    _ => unreachable!(),
                }
            }
            'n' => {
                index += 1;
                match chars[index] {
                    'e' => movements.push(Movement::NorthEast),
                    'w' => movements.push(Movement::NorthWest),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        };

        index += 1;
    }

    movements
}

fn parse_input(input: &str) -> Vec<Vec<Movement>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day24a(movement_lists: &Vec<Vec<Movement>>) -> usize {
    get_flipped_tiles(movement_lists).len()
}

fn day24b(movements: &Vec<Vec<Movement>>) -> usize {
    let mut flipped_tiles = get_flipped_tiles(movements);

    for _ in 0..100 {
        flipped_tiles = update_tiles(&flipped_tiles);
    }

    flipped_tiles.len()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let recipes = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day24a(&recipes);
    println!(
        "Day 24A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day24b(&recipes);
    println!(
        "Day 24B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
