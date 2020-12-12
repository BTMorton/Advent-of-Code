use std::fs::File;
use std::io::{BufReader, Read, Result};

struct SeatMap {
    height: usize,
    width: usize,
    map: Vec<Option<bool>>,
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    static TEST_INPUT: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn should_parse_a_seat_map() {
        assert_eq!(
            vec![
                Some(false),
                None,
                Some(false),
                None,
                Some(false),
                None,
                Some(false),
                None,
                Some(false)
            ],
            parse_seat_map(
                "L.L
.L.
L.L"
            )
            .map
        );
    }

    #[test]
    fn should_parse_file_input_to_seat_map() {
        let result = parse_seat_map(TEST_INPUT);
        assert_eq!(10, result.height);
        assert_eq!(10, result.width);
        assert!(result.map[0].is_some());
        assert_eq!(false, result.map[0].unwrap());
        assert!(result.map[1].is_none());
        assert!(result.map[11].is_some());
        assert!(result.map[17].is_none());
    }

    #[test]
    fn should_detect_taken_seats() {
        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|n| Some(n % 2 == 0)).collect(),
        };

        assert!(is_seat_taken(0, &map));
        assert!(!is_seat_taken(1, &map));
        assert!(is_seat_taken(2, &map));
        assert!(!is_seat_taken(3, &map));
        assert!(is_seat_taken(4, &map));
        assert!(!is_seat_taken(5, &map));
        assert!(is_seat_taken(6, &map));
        assert!(!is_seat_taken(7, &map));
        assert!(is_seat_taken(8, &map));
    }

    #[test]
    fn should_count_neighbours() {
        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|_| Some(true)).collect(),
        };
        let neighbours = find_all_neighbours(&map);

        assert_eq!(8, count_neighbours(4, &map, &neighbours));
        assert_eq!(3, count_neighbours(0, &map, &neighbours));
        assert_eq!(5, count_neighbours(1, &map, &neighbours));
        assert_eq!(5, count_neighbours(3, &map, &neighbours));
        assert_eq!(5, count_neighbours(7, &map, &neighbours));
        assert_eq!(5, count_neighbours(5, &map, &neighbours));
        assert_eq!(3, count_neighbours(8, &map, &neighbours));

        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|n| Some(n % 2 == 0)).collect(),
        };

        assert_eq!(4, count_neighbours(4, &map, &neighbours));
        assert_eq!(1, count_neighbours(0, &map, &neighbours));
        assert_eq!(3, count_neighbours(1, &map, &neighbours));
        assert_eq!(3, count_neighbours(3, &map, &neighbours));
        assert_eq!(3, count_neighbours(7, &map, &neighbours));
        assert_eq!(3, count_neighbours(5, &map, &neighbours));
        assert_eq!(1, count_neighbours(8, &map, &neighbours));
    }

    #[test]
    fn should_fill_an_empty_seat_with_no_neighbours() {
        let map = SeatMap {
            height: 5,
            width: 5,
            map: (0..5)
                .flat_map(|y| (0..5).map(move |x| (x, y)))
                .map(|(x, y)| Some(x == 0 || x == 4 || y == 0 || y == 4))
                .collect(),
        };
        let neighbours = find_all_neighbours(&map);

        let result = step(&map, &neighbours, 4);
        assert_eq!(true, result.0.map[12].unwrap_or(false));
    }

    #[test]
    fn should_not_fill_an_empty_seat_with_neighbours() {
        let map = SeatMap {
            height: 5,
            width: 5,
            map: (0..5)
                .flat_map(|y| (0..5).map(move |x| (x, y)))
                .map(|(x, y)| Some(x == 0 || x == 4 || y == 0 || y == 4))
                .collect(),
        };
        let neighbours = find_all_neighbours(&map);

        let result = step(&map, &neighbours, 4);
        assert_eq!(false, result.0.map[6].unwrap_or(false));
        assert_eq!(false, result.0.map[7].unwrap_or(false));
        assert_eq!(false, result.0.map[8].unwrap_or(false));
        assert_eq!(false, result.0.map[11].unwrap_or(false));
        assert_eq!(false, result.0.map[13].unwrap_or(false));
        assert_eq!(false, result.0.map[16].unwrap_or(false));
        assert_eq!(false, result.0.map[17].unwrap_or(false));
        assert_eq!(false, result.0.map[18].unwrap_or(false));
    }

    #[test]
    fn should_empty_a_taken_seat_with_over_4_neighbours() {
        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|_| Some(true)).collect(),
        };
        println!("here");
        let neighbours = find_all_neighbours(&map);

        println!("here");
        let result = step(&map, &neighbours, 4);
        assert_eq!(false, result.0.map[1].unwrap_or(false));
        assert_eq!(false, result.0.map[3].unwrap_or(false));
        assert_eq!(false, result.0.map[4].unwrap_or(false));
        assert_eq!(false, result.0.map[5].unwrap_or(false));
        assert_eq!(false, result.0.map[7].unwrap_or(false));

        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|n| Some(n % 2 == 0)).collect(),
        };

        let result = step(&map, &neighbours, 4);
        assert_eq!(false, result.0.map[4].unwrap_or(false));
    }

    #[test]
    fn should_not_empty_a_taken_seat_with_below_4_neighbours() {
        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|_| Some(true)).collect(),
        };
        let neighbours = find_all_neighbours(&map);
        println!("{:?}", neighbours);

        let result = step(&map, &neighbours, 4);
        assert_eq!(true, result.0.map[0].unwrap_or(false));
        assert_eq!(true, result.0.map[2].unwrap_or(false));
        assert_eq!(true, result.0.map[6].unwrap_or(false));
        assert_eq!(true, result.0.map[8].unwrap_or(false));

        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|n| Some(n % 2 == 0)).collect(),
        };

        let result = step(&map, &neighbours, 4);
        assert_eq!(true, result.0.map[0].unwrap_or(false));
        assert_eq!(true, result.0.map[2].unwrap_or(false));
        assert_eq!(true, result.0.map[6].unwrap_or(false));
        assert_eq!(true, result.0.map[8].unwrap_or(false));
    }

    #[test]
    fn should_find_neighbours() {
        let map = SeatMap {
            height: 3,
            width: 3,
            map: (0..9).map(|_| Some(true)).collect(),
        };
        assert_eq!(vec![1, 3, 4], find_neighbours(0, &map));
        assert_eq!(vec![0, 1, 2, 3, 5, 6, 7, 8], find_neighbours(4, &map));
        assert_eq!(vec![4, 5, 7], find_neighbours(8, &map));
        assert_eq!(vec![0, 2, 3, 4, 5], find_neighbours(1, &map));
        assert_eq!(vec![1, 2, 4, 7, 8], find_neighbours(5, &map));

        let map = SeatMap {
            height: 5,
            width: 5,
            map: (0..5)
                .flat_map(|y| (0..5).map(move |x| (x, y)))
                .map(|(x, y)| (x == 2 && y == 2) || (x == 0 || x == 4 || y == 0 || y == 4))
                .map(|seat| if seat { Some(false) } else { None })
                .collect(),
        };
        assert_eq!(vec![1, 5], find_neighbours(0, &map));
        assert_eq!(Vec::<usize>::new(), find_neighbours(11, &map));
        assert_eq!(Vec::<usize>::new(), find_neighbours(12, &map));
        assert_eq!(vec![1, 3], find_neighbours(2, &map));
    }

    #[test]
    fn should_find_nearest_seats() {
        let map = SeatMap {
            height: 5,
            width: 5,
            map: (0..5)
                .flat_map(|y| (0..5).map(move |x| (x, y)))
                .map(|(x, y)| (x == 2 && y == 2) || (x == 0 || x == 4 || y == 0 || y == 4))
                .map(|seat| if seat { Some(false) } else { None })
                .collect(),
        };

        assert_eq!(vec![1, 5, 12], find_nearest_seats(0, &map));
        assert_eq!(vec![0, 2, 5, 21, 19], find_nearest_seats(1, &map));
        assert_eq!(vec![1, 3, 10, 12, 14], find_nearest_seats(2, &map));
        assert_eq!(vec![2, 4, 15, 23, 9], find_nearest_seats(3, &map));
        assert_eq!(vec![3, 12, 9], find_nearest_seats(4, &map));
        assert_eq!(vec![0, 1, 9, 10, 23], find_nearest_seats(5, &map));
        assert_eq!(vec![3, 4, 5, 21, 14], find_nearest_seats(9, &map));
        assert_eq!(vec![5, 2, 12, 15, 22], find_nearest_seats(10, &map));
        assert_eq!(
            vec![0, 2, 4, 10, 14, 20, 22, 24],
            find_nearest_seats(12, &map)
        );
        assert_eq!(vec![2, 9, 12, 22, 19], find_nearest_seats(14, &map));
        assert_eq!(vec![10, 3, 19, 20, 21], find_nearest_seats(15, &map));
        assert_eq!(vec![1, 14, 15, 23, 24], find_nearest_seats(19, &map));
        assert_eq!(vec![15, 12, 21], find_nearest_seats(20, &map));
        assert_eq!(vec![15, 1, 9, 20, 22], find_nearest_seats(21, &map));
        assert_eq!(vec![10, 12, 14, 21, 23], find_nearest_seats(22, &map));
        assert_eq!(vec![5, 3, 19, 22, 24], find_nearest_seats(23, &map));
        assert_eq!(vec![12, 19, 23], find_nearest_seats(24, &map));
    }

    #[test]
    fn day11a_test() {
        assert_eq!(37, day11a(&parse_seat_map(TEST_INPUT)));
    }

    #[test]
    fn day11b_test() {
        assert_eq!(26, day11b(&parse_seat_map(TEST_INPUT)));
    }
}

fn find_nearest_seat(
    start_x: usize,
    start_y: usize,
    stride_x: isize,
    stride_y: isize,
    map: &SeatMap,
) -> Option<usize> {
    if stride_x == 0 && stride_y == 0 {
        return None;
    }

    let max_x = map.width as isize;
    let max_y = map.height as isize;
    let mut next_x = (start_x as isize) + stride_x;
    let mut next_y = (start_y as isize) + stride_y;
    let mut target = (next_y * max_x + next_x) as usize;
    let mut seat = None;

    while next_x >= 0 && next_y >= 0 && next_x < max_x && next_y < max_y && seat.is_none() {
        target = (next_y * max_x + next_x) as usize;
        seat = map.map[target];
        next_x += stride_x;
        next_y += stride_y;
    }

    if seat.is_some() {
        Some(target)
    } else {
        None
    }
}

fn find_nearest_seats(seat: usize, map: &SeatMap) -> Vec<usize> {
    if map.map[seat].is_none() {
        return vec![];
    }

    let x = seat % map.width;
    let y = seat / map.width;
    (-1..=1)
        .flat_map(|h| (-1..=1).map(move |w| (w, h)))
        .filter_map(|(w, h)| find_nearest_seat(x, y, w, h, map))
        .collect()
}

fn step(map: &SeatMap, neighbours: &Vec<Vec<usize>>, max_neighbours: usize) -> (SeatMap, bool) {
    let mut changed = false;
    let result = map
        .map
        .iter()
        .enumerate()
        .map(|(seat, _)| update_seat(seat, map, neighbours, max_neighbours))
        .map(|(taken, change)| {
            changed = changed || change;
            taken
        })
        .collect();

    // println!("changed {}", changed);
    (
        SeatMap {
            height: map.height,
            width: map.width,
            map: result,
        },
        changed,
    )
}

fn update_seat(
    seat: usize,
    map: &SeatMap,
    neighbours: &Vec<Vec<usize>>,
    max_neighbours: usize,
) -> (Option<bool>, bool) {
    if map.map[seat].is_none() {
        return (None, false);
    }

    let taken = is_seat_taken(seat, map);
    let neighbours = count_neighbours(seat, map, neighbours);

    let result = if taken {
        neighbours < max_neighbours
    } else {
        neighbours == 0
    };

    (Some(result), taken != result)
}

fn find_neighbours(seat: usize, map: &SeatMap) -> Vec<usize> {
    if map.map[seat].is_none() {
        return vec![];
    }

    let x = (seat % map.width) as isize;
    let y = (seat / map.width) as isize;
    (-1..=1)
        .flat_map(|h| (-1..=1).map(move |w| (w, h)))
        .filter(|&(w, h)| w != 0 || h != 0)
        .filter(|(w, h)| {
            x + w >= 0 && y + h >= 0 && x + w < map.width as isize && y + h < map.height as isize
        })
        .map(|(w, h)| (h * (map.width as isize)) + w)
        .map(|stride| (seat as isize + stride) as usize)
        .filter(|&seat| map.map[seat].is_some())
        .collect()
}

fn count_neighbours(seat: usize, map: &SeatMap, neighbours: &Vec<Vec<usize>>) -> usize {
    neighbours[seat]
        .iter()
        .filter(|&&neighbour| is_seat_taken(neighbour, map))
        .count()
}

fn is_seat_taken(target: usize, map: &SeatMap) -> bool {
    map.map[target].unwrap_or(false)
}

fn parse_seat_map(file: &str) -> SeatMap {
    let lines: Vec<&str> = file.split("\n").filter(|&line| line != "").collect();
    let width = lines[0].chars().count();

    let map: Vec<Option<bool>> = lines
        .iter()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                'L' => Some(false),
                '#' => Some(true),
                _ => None,
            })
        })
        .collect();

    SeatMap {
        height: lines.len(),
        width: width,
        map: map,
    }
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn find_all_neighbours(map: &SeatMap) -> Vec<Vec<usize>> {
    map.map
        .iter()
        .enumerate()
        .map(|(seat, _)| find_neighbours(seat, &map))
        .collect()
}

fn find_all_nearest_seats(map: &SeatMap) -> Vec<Vec<usize>> {
    map.map
        .iter()
        .enumerate()
        .map(|(seat, _)| find_nearest_seats(seat, &map))
        .collect()
}

fn day11a(map: &SeatMap) -> usize {
    let neighbours = find_all_neighbours(&map);

    let mut result = step(map, &neighbours, 4);
    while result.1 {
        result = step(&result.0, &neighbours, 4);
    }

    result
        .0
        .map
        .iter()
        .filter(|taken| taken.unwrap_or(false))
        .count()
}

fn day11b(map: &SeatMap) -> usize {
    let neighbours = find_all_nearest_seats(&map);

    let mut result = step(map, &neighbours, 5);
    while result.1 {
        result = step(&result.0, &neighbours, 5);
    }

    result
        .0
        .map
        .iter()
        .filter(|taken| taken.unwrap_or(false))
        .count()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let map = parse_seat_map(input.as_str());

    use std::time::Instant;
    let total = Instant::now();

    let result = day11a(&map);
    println!(
        "Day 11A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day11b(&map);
    println!(
        "Day 11B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
