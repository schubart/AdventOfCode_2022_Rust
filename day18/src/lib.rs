#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

use std::collections::HashSet;

type Scalar = isize;
type Point = (Scalar, Scalar, Scalar);

fn parse() -> HashSet<Point> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut split = line.split(',').map(|s| s.parse().unwrap());

            (
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            )
        })
        .collect()
}

fn neighbours((x, y, z): Point) -> [Point; 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

#[test]
fn test_part1() {
    let cubes = parse();
    let surface: usize = cubes
        .iter()
        .map(|cube| {
            neighbours(*cube)
                .iter()
                .filter(|neighbour| !cubes.contains(neighbour))
                .count()
        })
        .sum();

    assert_eq!(4_636, surface);
}

fn is_outside(point: Point, min: Point, max: Point) -> bool {
    point.0 < min.0
        || point.0 > max.0
        || point.1 < min.1
        || point.1 > max.1
        || point.2 < min.2
        || point.2 > max.2
}

#[test]
fn test_part2() {
    let cubes = parse();
    let min = (
        cubes.iter().map(|cube| cube.0).min().unwrap() - 1,
        cubes.iter().map(|cube| cube.1).min().unwrap() - 1,
        cubes.iter().map(|cube| cube.2).min().unwrap() - 1,
    );
    let max = (
        cubes.iter().map(|cube| cube.0).max().unwrap() + 1,
        cubes.iter().map(|cube| cube.1).max().unwrap() + 1,
        cubes.iter().map(|cube| cube.2).max().unwrap() + 1,
    );

    let mut surface = 0;
    let mut to_visit = vec![min];
    let mut visited = HashSet::new();
    while let Some(point) = to_visit.pop() {
        if is_outside(point, min, max) || !visited.insert(point) {
            continue;
        }

        for neighbour in neighbours(point) {
            if cubes.contains(&neighbour) {
                surface += 1;
            } else {
                to_visit.push(neighbour);
            }
        }
    }

    assert_eq!(2_572, surface);
}
