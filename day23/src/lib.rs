#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::collections::HashMap;
use std::collections::HashSet;

type Point = (isize, isize);
type Points = HashSet<Point>;
type Direction = [Point; 3]; // Target and its two neighbours.
type Directions = [Direction; 4];

const NEIGHBOURS: [Point; 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const DIRECTIONS: Directions = [
    [(0, -1), (-1, -1), (1, -1)], // North
    [(0, 1), (-1, 1), (1, 1)],    // South
    [(-1, 0), (-1, -1), (-1, 1)], // West
    [(1, 0), (1, -1), (1, 1)],    // East
];

fn parse() -> Points {
    include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '#').then_some((x, y)))
        })
        .map(|(x, y)| (isize::try_from(x).unwrap(), isize::try_from(y).unwrap()))
        .collect()
}

fn plus(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

fn intention(elf: Point, elves: &Points, directions: &Directions) -> Point {
    // No neighbours? Elf remains where it is.
    if NEIGHBOURS.iter().all(|n| !elves.contains(&plus(elf, *n))) {
        return elf;
    }

    // Try moving in each direction.
    for direction in directions {
        // No neighbours in target and in the two spots next to it? Go there.
        if direction.iter().all(|n| !elves.contains(&plus(elf, *n))) {
            return plus(elf, direction[0]);
        }
    }

    // Could not move? Remain.
    elf
}

fn simulate(elves: &mut Points, directions: &mut Directions) -> bool {
    // Keep track of how many elves want to move to a target.
    let mut intentions = HashMap::new();
    for elf in &*elves {
        let intention = intention(*elf, elves, directions);
        *intentions.entry(intention).or_insert(0) += 1;
    }

    // Move each elf.
    let mut next_elves = elves
        .iter()
        .map(|&elf| {
            let intention = intention(elf, elves, directions);
            if intentions[&intention] == 1 {
                // It's the only elf trying to move there? Do it.
                intention
            } else {
                // Other elves intend to move there, too? Remain.
                elf
            }
        })
        .collect();
    std::mem::swap(elves, &mut next_elves);

    // Directions change after each round.
    directions.rotate_left(1);

    // True if there was some movement.
    *elves != next_elves
}

#[test]
fn test_part1() {
    let mut elves = parse();
    let mut directions = DIRECTIONS;

    for _ in 0..10 {
        simulate(&mut elves, &mut directions);
    }

    let min_x = elves.iter().map(|e| e.0).min().unwrap();
    let max_x = elves.iter().map(|e| e.0).max().unwrap();
    let min_y = elves.iter().map(|e| e.1).min().unwrap();
    let max_y = elves.iter().map(|e| e.1).max().unwrap();
    let elves_len = isize::try_from(elves.len()).unwrap();
    let empty = (1 + max_x - min_x) * (1 + max_y - min_y) - elves_len;

    assert_eq!(4_109, empty);
}

#[test]
fn test_part2() {
    let mut elves = parse();
    let mut directions = DIRECTIONS;

    #[allow(clippy::maybe_infinite_iter)]
    let rounds = (1..).find(|_| !simulate(&mut elves, &mut directions));

    assert_eq!(Some(1_055), rounds);
}
