#![no_std]
#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

// https://users.rust-lang.org/t/solved-best-way-to-find-largest-three-values-in-unsorted-slice/34754
fn sum_of_biggest(count: usize) -> u32 {
    let mut top = [0, 0, 0];

    let mut current = 0;
    for line in include_str!("input.txt").lines() {
        if line.is_empty() {
            if current > top[0] {
                top[2] = top[1];
                top[1] = top[0];
                top[0] = current;
            } else if current > top[1] {
                top[2] = top[1];
                top[1] = current;
            } else if current > top[2] {
                top[2] = current;
            }
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    top.iter().take(count).sum()
}

#[test]
fn test_part1() {
    assert_eq!(72_718, sum_of_biggest(1));
}

#[test]
fn test_part2() {
    assert_eq!(213_089, sum_of_biggest(3));
}
