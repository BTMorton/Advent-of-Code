static SUBJECT_NUMBER: usize = 7;
static MODULUS_NUMBER: usize = 20201227;

#[cfg(test)]
mod day25_tests {
    use super::*;

    #[test]
    fn should_calculate_the_loop_size() {
        assert_eq!(8, calculate_loop_size(5764801));
        assert_eq!(11, calculate_loop_size(17807724));
    }

    #[test]
    fn day25a_test() {
        assert_eq!(14897079, day25a(&[5764801, 17807724]))
    }
}

fn apply_step(value: usize, subject_number: usize) -> usize {
    (value * subject_number) % MODULUS_NUMBER
}

fn apply_loop(loop_size: usize, subject_number: usize) -> usize {
    let mut result = 1;

    for _ in 0..loop_size {
        result = apply_step(result, subject_number);
    }

    result
}

fn calculate_loop_size(input: usize) -> usize {
    let mut result = 1;
    let mut loop_size = 0;

    while result != input {
        result = apply_step(result, SUBJECT_NUMBER);
        loop_size += 1;
    }

    loop_size
}

fn day25a(input: &[usize; 2]) -> usize {
    let loop_size = calculate_loop_size(input[0]);

    apply_loop(loop_size, input[1])
}

fn main() {
    let input = [9232416, 14144084];

    use std::time::Instant;
    let total = Instant::now();

    let result = day25a(&input);
    println!(
        "Day 25A - {:?} ({:.2}ms)",
        result,
        total.elapsed().as_millis()
    );
}
