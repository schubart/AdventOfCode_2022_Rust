#![no_std]
#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

type Bits = u64;

fn bits(line: &str) -> Bits {
    line.chars()
        .map(|c| match c {
            'a'..='z' => c as u32 - 'a' as u32,
            'A'..='Z' => c as u32 - 'A' as u32 + 26,
            _ => panic!("{c}"),
        })
        .fold(0, |bits, bit| bits | 1 << bit)
}

#[test]
fn test_part1() {
    let total = include_str!("input.txt")
        .lines()
        .map(|line| {
            assert!(line.len() % 2 == 0);
            let (part1, part2) = line.split_at(line.len() / 2);

            let common = bits(part1) & bits(part2);
            assert!(common.count_ones() == 1);
            Bits::BITS - common.leading_zeros()
        })
        .sum::<u32>();

    assert_eq!(7_737, total);
}

#[test]
fn test_part2() {
    let mut total = 0;

    let mut lines = include_str!("input.txt").lines().peekable();
    while lines.peek().is_some() {
        let bits1 = bits(lines.next().unwrap());
        let bits2 = bits(lines.next().unwrap());
        let bits3 = bits(lines.next().unwrap());

        let common = bits1 & bits2 & bits3;
        assert!(common.count_ones() == 1);
        total += Bits::BITS - common.leading_zeros();
    }

    assert_eq!(2_697, total);
}
