use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

#[cfg(test)]
mod day3_tests {
    use super::*;

    static TEST_INPUT: &str = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn should_return_true_if_on_a_tree() {
        assert_eq!(true, is_on_tree(".#....#..#.", 2, 3).unwrap());
    }

    #[test]
    fn should_return_false_if_not_on_a_tree() {
        assert_eq!(false, is_on_tree("#...#...#..", 1, 3).unwrap());
    }

    #[test]
    fn should_return_true_if_on_a_tree_past_the_end_of_the_line() {
        assert_eq!(true, is_on_tree(".#........#", 7, 3).unwrap());
    }

    #[test]
    fn should_return_false_if_not_on_a_tree_past_the_end_of_the_line() {
        assert_eq!(false, is_on_tree(".#.#.#....#", 6, 3).unwrap());
    }

    #[test]
    fn should_count_trees_on_with_move_1_1() {
        let lines = TEST_INPUT.split("\n").filter(|&line| line != "").collect();
        assert_eq!(2, count_trees(lines, 1, 1));
    }

    #[test]
    fn should_count_trees_on_with_move_1_3() {
        let lines = TEST_INPUT.split("\n").filter(|&line| line != "").collect();
        assert_eq!(7, count_trees(lines, 1, 3));
    }

    #[test]
    fn should_count_trees_on_with_move_1_5() {
        let lines = TEST_INPUT.split("\n").filter(|&line| line != "").collect();
        assert_eq!(3, count_trees(lines, 1, 5));
    }

    #[test]
    fn should_count_trees_on_with_move_1_7() {
        let lines = TEST_INPUT.split("\n").filter(|&line| line != "").collect();
        assert_eq!(4, count_trees(lines, 1, 7));
    }

    #[test]
    fn should_count_trees_on_with_move_2_1() {
        let lines = TEST_INPUT.split("\n").filter(|&line| line != "").collect();
        assert_eq!(2, count_trees(lines, 2, 1));
    }

    #[test]
    fn day3a_test() {
        assert_eq!(7, day3a(TEST_INPUT));
    }

    #[test]
    fn day3b_test() {
        assert_eq!(336, day3b(TEST_INPUT));
    }
}

fn is_on_tree(line: &str, line_no: usize, stride: usize) -> Option<bool> {
    if line_no == 0 {
        return Some(false);
    }
    let pos = (line_no * stride) % line.len();
    Some(line.chars().nth(pos)? == '#')
}

fn count_trees(lines: Vec<&str>, v_stride: usize, h_stride: usize) -> usize {
    lines
        .iter()
        .step_by(v_stride)
        .enumerate()
        .map(|(i, line)| is_on_tree(line, i, h_stride))
        .filter_map(|x| x)
        .filter(|&x| x)
        .count()
}

fn day3a(input: &str) -> usize {
    let lines = input.split("\n").filter(|&line| line != "").collect();
    count_trees(lines, 1, 3)
}

fn day3b(input: &str) -> usize {
    let lines: Vec<&str> = input.split("\n").filter(|&line| line != "").collect();
    count_trees(lines.clone(), 1, 1)
        * count_trees(lines.clone(), 1, 3)
        * count_trees(lines.clone(), 1, 5)
        * count_trees(lines.clone(), 1, 7)
        * count_trees(lines.clone(), 2, 1)
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

    let result = day3a(input.as_str());
    println!("Day 3A - {}", result);

    let result = day3b(input.as_str());
    println!("Day 3B - {}", result);

    Ok(())
}
