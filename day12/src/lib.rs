#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::collections::HashMap;

type Point = (isize, isize);
type Level = u8;

fn length(mut levels: HashMap<Point, Level>, start: Point, end: Point) -> Option<usize> {
    let mut to_visit = vec![start];
    let mut length = 0;
    while !to_visit.is_empty() {
        let mut to_visit_next = vec![];

        for visit in to_visit {
            if let Some(level) = levels.remove(&visit) {
                if visit == end {
                    return Some(length);
                }

                for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let neighbour = (visit.0 + delta.0, visit.1 + delta.1);
                    if let Some(neighbour_level) = levels.get(&neighbour) {
                        if level + 1 >= *neighbour_level {
                            to_visit_next.push(neighbour);
                        }
                    }
                }
            }
        }

        length += 1;
        to_visit = to_visit_next;
    }

    None
}

#[test]
fn test_part1() {
    let mut levels: HashMap<Point, Level> = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    let start = *levels.iter().find(|e| *e.1 == b'S').map(|e| e.0).unwrap();
    levels.insert(start, b'a');
    let end = *levels.iter().find(|e| *e.1 == b'E').map(|e| e.0).unwrap();
    levels.insert(end, b'z');

    let length = length(levels, start, end);
    assert_eq!(Some(484), length);
}

#[test]
fn test_part2() {
    let mut levels: HashMap<Point, Level> = include_str!("input.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    let start = *levels.iter().find(|e| *e.1 == b'S').map(|e| e.0).unwrap();
    levels.insert(start, b'a');
    let end = *levels.iter().find(|e| *e.1 == b'E').map(|e| e.0).unwrap();
    levels.insert(end, b'z');

    let mut min_length = 484;
    for (point, level) in &levels {
        if level == &b'a' {
            if let Some(length) = length(levels.clone(), *point, end) {
                min_length = std::cmp::min(length, min_length);
                dbg!(min_length);
            }
        }
    }
    assert_eq!(478, min_length);
}
