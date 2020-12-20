use std::collections::HashMap;

#[cfg(test)]
mod day15_test {
    use super::*;

    #[test]
    fn day15a_test() {
        assert_eq!(436, day15a(&vec![0, 3, 6]));
        assert_eq!(1, day15a(&vec![1, 3, 2]));
        assert_eq!(10, day15a(&vec![2, 1, 3]));
        assert_eq!(27, day15a(&vec![1, 2, 3]));
        assert_eq!(78, day15a(&vec![2, 3, 1]));
        assert_eq!(438, day15a(&vec![3, 2, 1]));
        assert_eq!(1836, day15a(&vec![3, 1, 2]));
    }

    #[test]
    fn day15b_test() {
        assert_eq!(175594, day15b(&vec![0, 3, 6]));
        assert_eq!(2578, day15b(&vec![1, 3, 2]));
        assert_eq!(3544142, day15b(&vec![2, 1, 3]));
        assert_eq!(261214, day15b(&vec![1, 2, 3]));
        assert_eq!(6895259, day15b(&vec![2, 3, 1]));
        assert_eq!(18, day15b(&vec![3, 2, 1]));
        assert_eq!(362, day15b(&vec![3, 1, 2]));
    }
}

fn play_game(starting_numbers: &Vec<usize>, last_turn: usize) -> usize {
    let mut last_seen_turns: Vec<Option<usize>> = vec![None; last_turn];
    let mut last_number = *starting_numbers.last().unwrap();

    for turn in 0..starting_numbers.len() - 1 {
        last_seen_turns[starting_numbers[turn]] = Some(turn);
    }

    for turn in (starting_numbers.len() - 1)..(last_turn - 1) {
        let next_number = match last_seen_turns[last_number] {
            Some(last_turn) => turn - last_turn,
            None => 0,
        };

        last_seen_turns[last_number] = Some(turn);
        last_number = next_number;
    }

    last_number
}

fn day15a(starting_numbers: &Vec<usize>) -> usize {
    play_game(starting_numbers, 2020)
}

fn day15b(starting_numbers: &Vec<usize>) -> usize {
    play_game(starting_numbers, 30000000)
}

fn main() {
    let input = vec![6, 19, 0, 5, 7, 13, 1];
    use std::time::Instant;
    let total = Instant::now();

    let result = day15a(&input);
    println!(
        "Day 15A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day15b(&input);
    println!(
        "Day 15B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());
}
