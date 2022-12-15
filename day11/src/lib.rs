#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::fmt::Debug;
use std::str::FromStr;
use std::str::Lines;

type Item = i64;
type Operand = i64;
type Operation = Box<dyn Fn(Item) -> Item>;

struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: i64,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

// Helpers for parsing:

fn strip_next<'a>(lines: &mut Lines<'a>, prefix: &str) -> &'a str {
    lines.next().unwrap().strip_prefix(prefix).unwrap()
}

fn parse_next<F>(lines: &mut Lines, prefix: &str) -> F
where
    F: FromStr,
    <F as FromStr>::Err: Debug,
{
    strip_next(lines, prefix).parse().unwrap()
}

fn parse_items(lines: &mut Lines, prefix: &str) -> Vec<Item> {
    strip_next(lines, prefix)
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect()
}

fn parse_operation(lines: &mut Lines, prefix: &str) -> Operation {
    let operands: Vec<_> = strip_next(lines, prefix).split(' ').collect();

    match operands[..] {
        ["old", "*", "old"] => Box::new(|x| x * x),

        ["old", "*", y] => {
            let y: Operand = y.parse().unwrap();
            Box::new(move |x| x * y)
        }

        ["old", "+", y] => {
            let y: Operand = y.parse().unwrap();
            Box::new(move |x| x + y)
        }

        _ => panic!("{operands:?}"),
    }
}

fn parse() -> Vec<Monkey> {
    let mut result = Vec::new();
    let mut lines = include_str!("input.txt").lines();
    loop {
        lines.next(); // Skip monkey ID.

        result.push(Monkey {
            items: parse_items(&mut lines, "  Starting items: "),
            operation: parse_operation(&mut lines, "  Operation: new = "),
            test: parse_next(&mut lines, "  Test: divisible by "),
            if_true: parse_next(&mut lines, "    If true: throw to monkey "),
            if_false: parse_next(&mut lines, "    If false: throw to monkey "),
            inspections: 0,
        });

        if lines.next().is_none() {
            break;
        }
    }

    result
}

fn solve<R>(mut monkeys: Vec<Monkey>, rounds: usize, reduction: R) -> usize
where
    R: Fn(Item) -> Item,
{
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            // Apply math to items.
            let items: Vec<_> = monkey
                .items
                .drain(..)
                .map(|item| (monkey.operation)(item))
                .map(&reduction)
                .collect();

            // Count inspections.
            monkey.inspections += items.len();

            // Throw items to other monkyes.
            let test = monkey.test;
            let if_true = monkey.if_true;
            let if_false = monkey.if_false;
            for item in items {
                let target = if item % test == 0 { if_true } else { if_false };
                monkeys[target].items.push(item);
            }
        }
    }

    // Extract inspection counts.
    let mut inspections: Vec<_> = monkeys.iter().map(|m| m.inspections).collect();
    // Return product of top two inspection counts.
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections.iter().take(2).product()
}

#[test]
fn test_part1() {
    let monkeys = parse();
    let result = solve(monkeys, 20, |x| x / 3);
    assert_eq!(95_472, result);
}

#[test]
fn test_part2() {
    let monkeys = parse();
    let lcd: Operand = monkeys.iter().map(|m| m.test).product();
    let result = solve(monkeys, 10_000, |x| x % lcd);
    assert_eq!(17_926_061_332, result);
}
