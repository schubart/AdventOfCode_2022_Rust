#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]
#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

// Original index and value.
type IndexAndValue = (usize, isize);

fn decode(key: isize, rounds: usize) -> isize {
    // Parse numbers, multiply with key, keep track of original index.
    let mut numbers: Vec<IndexAndValue> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse::<isize>().unwrap() * key)
        .enumerate()
        .collect();

    for _ in 0..rounds {
        // For each number in their original order.
        for original_index in 0..numbers.len() {
            // Get current index of the number with the original index.
            let index = numbers.iter().position(|x| x.0 == original_index).unwrap();
            // Get the value.
            let value = numbers[index].1;
            // New index is current index plus value.
            let new_index = index as isize + value;
            // Truncate. Use euclid because new_index might be negative.
            // Length minus 1 because of problem statement: Moving an element
            // by (n - 1) places in a list of length n leaves list unchanged.
            let new_index = new_index.rem_euclid(numbers.len() as isize - 1);

            // Pull out number from current index and insert it at new index.
            let tmp = numbers.remove(index);
            numbers.insert(new_index as usize, tmp);
        }
    }

    // Calculate result.
    let zero = numbers.iter().position(|x| x.1 == 0).unwrap();
    let x1 = numbers[(zero + 1_000) % numbers.len()].1;
    let x2 = numbers[(zero + 2_000) % numbers.len()].1;
    let x3 = numbers[(zero + 3_000) % numbers.len()].1;
    x1 + x2 + x3
}

#[test]
fn test_part1() {
    assert_eq!(8_302, decode(1, 1));
}

#[test]
fn test_part2() {
    assert_eq!(656_575_624_777, decode(811_589_153, 10));
}
