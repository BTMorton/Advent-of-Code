type Cups = Vec<usize>;
type CupInput = [usize; 9];

#[cfg(test)]
mod day23_tests {
    use super::*;

    static TEST_INPUT: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn should_convert_an_input_to_vec_linked_list() {
        assert_eq!(
            vec![0, 2, 5, 8, 6, 4, 7, 3, 9, 1],
            to_linked_list(&TEST_INPUT)
        );
    }

    #[test]
    fn should_move_cups_in_a_turn() {
        let mut state = to_linked_list(&TEST_INPUT);
        let next_cup = play_turn(&mut state, 3);
        assert_eq!(2, next_cup);
        assert_eq!(to_linked_list(&[2, 8, 9, 1, 5, 4, 6, 7, 3,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(5, next_cup);
        assert_eq!(to_linked_list(&[5, 4, 6, 7, 8, 9, 1, 3, 2,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(8, next_cup);
        assert_eq!(to_linked_list(&[8, 9, 1, 3, 4, 6, 7, 2, 5,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(4, next_cup);
        assert_eq!(to_linked_list(&[4, 6, 7, 9, 1, 3, 2, 5, 8,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(1, next_cup);
        assert_eq!(to_linked_list(&[1, 3, 6, 7, 9, 2, 5, 8, 4,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(9, next_cup);
        assert_eq!(to_linked_list(&[9, 3, 6, 7, 2, 5, 8, 4, 1,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(2, next_cup);
        assert_eq!(to_linked_list(&[2, 5, 8, 3, 6, 7, 4, 1, 9,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(6, next_cup);
        assert_eq!(to_linked_list(&[6, 7, 4, 1, 5, 8, 3, 9, 2,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(5, next_cup);
        assert_eq!(to_linked_list(&[5, 7, 4, 1, 8, 3, 9, 2, 6,]), state);

        let next_cup = play_turn(&mut state, next_cup);
        assert_eq!(8, next_cup);
        assert_eq!(to_linked_list(&[8, 3, 7, 4, 1, 9, 2, 6, 5,]), state);
    }

    #[test]
    fn day23a_test() {
        assert_eq!("67384529".to_string(), day23a(&TEST_INPUT));
    }

    #[test]
    fn day23b_test() {
        assert_eq!(149245887792, day23b(&TEST_INPUT));
    }
}

fn to_linked_list(cups: &CupInput) -> Cups {
    let mut state = vec![0; cups.len() + 1];

    for i in 0..cups.len() {
        state[cups[i]] = cups[(i + 1) % cups.len()];
    }

    state
}

fn play_turn(state: &mut Cups, current_cup: usize) -> usize {
    let start_cup_to_move = state[current_cup];
    let mid_cup_to_move = state[start_cup_to_move];
    let end_cup_to_move = state[mid_cup_to_move];
    let next_cup = state[end_cup_to_move];

    let mut target_cup = current_cup;

    loop {
        if target_cup == 1 {
            target_cup = state.len();
        }

        target_cup -= 1;

        if target_cup != start_cup_to_move
            && target_cup != mid_cup_to_move
            && target_cup != end_cup_to_move
        {
            break;
        }
    }

    state[current_cup] = next_cup;

    let cup_to_link = state[target_cup];
    state[target_cup] = start_cup_to_move;
    state[end_cup_to_move] = cup_to_link;

    next_cup
}

fn day23a(cups: &CupInput) -> String {
    let mut game_state = to_linked_list(&cups);
    let mut next_cup = cups[0];

    for _ in 0..100 {
        next_cup = play_turn(&mut game_state, next_cup);
    }

    let mut next_index = game_state[1];
    let mut game_result = vec![];

    while next_index != 1 {
        game_result.push((next_index as u8 + b'0') as char);
        next_index = game_state[next_index];
    }

    game_result.iter().collect()
}

fn day23b(cups: &CupInput) -> usize {
    let mut game_state = to_linked_list(&cups);
    let mut next_cup = cups[0];

    for i in cups.len() + 1..=1000000 {
        game_state.push(i + 1);
    }

    game_state[cups[cups.len() - 1]] = cups.len() + 1;
    game_state[1000000] = cups[0];

    for _ in 0..10000000 {
        next_cup = play_turn(&mut game_state, next_cup);
    }

    let star1 = game_state[1];
    let star2 = game_state[star1];

    star1 * star2
}

fn main() {
    let input = [5, 8, 9, 1, 7, 4, 2, 6, 3];

    use std::time::Instant;
    let total = Instant::now();

    let result = day23a(&input);
    println!(
        "Day 23A - {:?} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );

    let part2 = Instant::now();
    let result = day23b(&input);
    println!(
        "Day 23B - {} ({:.2}ms)",
        result,
        part2.elapsed().as_millis()
    );
    println!("Total ({:.2}ms)", total.elapsed().as_millis());
}
