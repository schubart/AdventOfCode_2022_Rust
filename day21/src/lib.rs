#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::collections::HashMap;

type Monkey = &'static str;
type Number = isize;
type Numbers = HashMap<Monkey, Number>;
type Operation = &'static str;
type Formula = (Operation, Monkey, Monkey);
type Formulas = HashMap<Monkey, Formula>;

fn parse() -> (Numbers, Formulas) {
    let mut numbers = HashMap::new();
    let mut formulas = HashMap::new();

    for line in include_str!("input.txt").lines() {
        let split: Vec<_> = line.split(&[':', ' '][..]).collect();
        if split.len() == 3 {
            numbers.insert(split[0], split[2].parse().unwrap());
        } else {
            let formula = (split[3], split[2], split[4]);
            formulas.insert(split[0], formula);
        }
    }

    (numbers, formulas)
}

fn evaluate(numbers: &mut Numbers, formulas: &mut Formulas) {
    loop {
        let formulas_len = formulas.len();

        formulas.retain(|monkey, (op, lhs, rhs)| {
            let number = match (*op, numbers.get(lhs), numbers.get(rhs)) {
                ("+", Some(lhs), Some(rhs)) => lhs + rhs,
                ("-", Some(lhs), Some(rhs)) => lhs - rhs,
                ("*", Some(lhs), Some(rhs)) => lhs * rhs,
                ("/", Some(lhs), Some(rhs)) => lhs / rhs,
                _ => return true, // Retain this formula, evaluate it later.
            };

            // Remember the number to which the formula was evaluated.
            numbers.insert(monkey, number);

            // Formula was evaluated. Remove (i.e. do not retain) it.
            false
        });

        // Stop if no new formulas were evaluated.
        if formulas.len() == formulas_len {
            break;
        }
    }
}

#[test]
fn test_part1() {
    let (mut numbers, mut formulas) = parse();
    evaluate(&mut numbers, &mut formulas);
    assert_eq!(324_122_188_240_430, numbers["root"]);
}

#[test]
fn test_part2() {
    let (mut numbers, mut formulas) = parse();
    // Patch up input as per part 2 description.
    formulas.get_mut("root").unwrap().0 = "=";
    numbers.remove("humn");
    // Evaluate as much as possible.
    evaluate(&mut numbers, &mut formulas);

    let mut search = "root";
    let mut result = 0; // Initial value does not matter.
    while search != "humn" {
        let (op, lhs, rhs) = &formulas[search];
        (search, result) = match (*op, numbers.get(lhs), numbers.get(rhs)) {
            ("=", None, Some(x)) => (lhs, *x),
            ("=", Some(x), None) => (rhs, *x),
            ("+", None, Some(x)) => (lhs, result - x),
            ("+", Some(x), None) => (rhs, result - x),
            ("-", None, Some(x)) => (lhs, result + x),
            ("-", Some(x), None) => (rhs, x - result),
            ("*", None, Some(x)) => (lhs, result / x),
            ("*", Some(x), None) => (rhs, result / x),
            ("/", None, Some(x)) => (lhs, result * x),
            ("/", Some(x), None) => (rhs, x / result),
            _ => panic!(),
        };
    }

    assert_eq!(3_412_650_897_405, result);
}
