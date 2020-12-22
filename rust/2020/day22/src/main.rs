use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, Read, Result};

type Hands = (Vec<usize>, Vec<usize>);

#[cfg(test)]
mod day22_tests {
    use super::*;

    static TEST_INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    fn get_test_input() -> Hands {
        (vec![9, 2, 6, 3, 1], vec![5, 8, 4, 7, 10])
    }

    #[test]
    fn should_parse_input_to_hands() {
        assert_eq!(get_test_input(), parse_input(TEST_INPUT));
    }

    #[test]
    fn should_play_a_round_of_combat() {
        let result = play_round(&get_test_input());
        assert_eq!((vec![2, 6, 3, 1, 9, 5], vec![8, 4, 7, 10]), result);
        let result = play_round(&result);
        assert_eq!((vec![6, 3, 1, 9, 5], vec![4, 7, 10, 8, 2]), result);
    }

    #[test]
    fn should_play_a_recursive_round() {
        let result = play_recursive_round(&(vec![4, 9, 8, 5, 2], vec![3, 10, 1, 7, 6]));
        assert_eq!((vec![9, 8, 5, 2], vec![10, 1, 7, 6, 3, 4]), result)
    }

    #[test]
    fn should_handle_an_infinite_recursive_game() {
        let result = play_recursive_game(&(vec![43, 19], vec![2, 29, 14]));
        assert_eq!((vec![43, 19], vec![]), result)
    }

    #[test]
    fn day22a_test() {
        assert_eq!(306, day22a(&get_test_input()));
    }

    #[test]
    fn day22b_test() {
        assert_eq!(291, day22b(&get_test_input()));
    }
}

fn play_recursive_round(hands: &Hands) -> Hands {
    let player1_card = hands.0[0];
    let player2_card = hands.1[0];

    let mut player1_hand: Vec<usize> = hands.0[1..].into_iter().map(|&n| n).collect();
    let mut player2_hand: Vec<usize> = hands.1[1..].into_iter().map(|&n| n).collect();

    let player1_win = if player1_card <= player1_hand.len() && player2_card <= player2_hand.len() {
        let subgame_player1 = player1_hand[0..player1_card]
            .into_iter()
            .map(|&n| n)
            .collect();
        let subgame_player2 = player2_hand[0..player2_card]
            .into_iter()
            .map(|&n| n)
            .collect();
        let subgame_result = play_recursive_game(&(subgame_player1, subgame_player2));

        subgame_result.0.len() > 0
    } else {
        player1_card > player2_card
    };

    if player1_win {
        player1_hand.push(player1_card);
        player1_hand.push(player2_card);
    } else {
        player2_hand.push(player2_card);
        player2_hand.push(player1_card);
    }

    (player1_hand, player2_hand)
}

fn play_recursive_game(hands: &Hands) -> Hands {
    let mut current_hands = hands.clone();
    let mut seen_hands = HashSet::<Hands>::new();

    while current_hands.0.len() > 0 && current_hands.1.len() > 0 {
        if seen_hands.contains(&current_hands) {
            return (current_hands.0, vec![]);
        }

        seen_hands.insert(current_hands.clone());
        current_hands = play_recursive_round(&current_hands);
    }

    current_hands
}

fn play_round(hands: &Hands) -> Hands {
    let player1_card = hands.0[0];
    let player2_card = hands.1[0];

    let mut player1_hand: Vec<usize> = hands.0[1..].into_iter().map(|&n| n).collect();
    let mut player2_hand: Vec<usize> = hands.1[1..].into_iter().map(|&n| n).collect();

    if player1_card > player2_card {
        player1_hand.push(player1_card);
        player1_hand.push(player2_card);
    } else {
        player2_hand.push(player2_card);
        player2_hand.push(player1_card);
    }

    (player1_hand, player2_hand)
}

fn parse_hand(hand: &str) -> Vec<usize> {
    hand.lines()
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

fn parse_input(input: &str) -> Hands {
    let mut hands = input.split("\n\n").take(2).map(|hand| parse_hand(hand));

    (hands.next().unwrap(), hands.next().unwrap())
}

fn read_file(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut input = String::new();
    buf_reader.read_to_string(&mut input)?;

    Ok(input)
}

fn day22a(hands: &Hands) -> usize {
    let mut current_hands = hands.clone();

    while current_hands.0.len() > 0 && current_hands.1.len() > 0 {
        current_hands = play_round(&current_hands);
    }

    let winning_hand = if current_hands.0.len() == 0 {
        current_hands.1
    } else {
        current_hands.0
    };

    winning_hand
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| n * (i + 1))
        .sum()
}

fn day22b(hands: &Hands) -> usize {
    let game_result = play_recursive_game(hands);

    let winning_hand = if game_result.0.len() == 0 {
        game_result.1
    } else {
        game_result.0
    };

    winning_hand
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, n)| n * (i + 1))
        .sum()
}

fn main() -> Result<()> {
    let input = read_file("input")?;
    let recipes = parse_input(&input);

    use std::time::Instant;
    let total = Instant::now();

    let result = day22a(&recipes);
    println!(
        "Day 22A - {} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day22b(&recipes);
    println!(
        "Day 22B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());

    Ok(())
}
