#![no_std]
#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

fn parse(line: &str) -> (u32, u32, u32, u32) {
    let mut split = line.split(&['-', ','][..]);

    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

#[test]
fn test_part1() {
    let count = include_str!("input.txt")
        .lines()
        .map(parse)
        .filter(|(s1, e1, s2, e2)| (s1 >= s2 && e1 <= e2) || (s2 >= s1 && e2 <= e1))
        .count();

    assert_eq!(605, count);
}

#[test]
fn test_part2() {
    let count = include_str!("input.txt")
        .lines()
        .map(parse)
        .filter(|(s1, e1, s2, e2)| s1 <= e2 && e1 >= s2)
        .count();

    assert_eq!(914, count);
}
