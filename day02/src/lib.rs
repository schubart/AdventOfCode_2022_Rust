#![no_std]
#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

const LOSE: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

#[test]
fn test_part1() {
    let score: u32 = include_str!("input.txt")
        .lines()
        .map(|line| match line {
            "A X" => DRAW + ROCK,
            "A Y" => WIN + PAPER,
            "A Z" => LOSE + SCISSORS,

            "B X" => LOSE + ROCK,
            "B Y" => DRAW + PAPER,
            "B Z" => WIN + SCISSORS,

            "C X" => WIN + ROCK,
            "C Y" => LOSE + PAPER,
            "C Z" => DRAW + SCISSORS,

            _ => panic!("{line}"),
        })
        .sum();

    assert_eq!(15_523, score);
}

#[test]
fn test_part2() {
    let score: u32 = include_str!("input.txt")
        .lines()
        .map(|line| match line {
            "A X" => LOSE + SCISSORS,
            "A Y" => DRAW + ROCK,
            "A Z" => WIN + PAPER,

            "B X" => LOSE + ROCK,
            "B Y" => DRAW + PAPER,
            "B Z" => WIN + SCISSORS,

            "C X" => LOSE + PAPER,
            "C Y" => DRAW + SCISSORS,
            "C Z" => WIN + ROCK,

            _ => panic!("{line}"),
        })
        .sum();

    assert_eq!(15_702, score);
}
