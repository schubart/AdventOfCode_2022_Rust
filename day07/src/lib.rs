#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

fn sizes() -> Vec<u64> {
    // All directory sizes observed.
    let mut result = Vec::new();
    // The size of each parent of the current directory.
    let mut parents = Vec::new();

    // Assumption: Every directory is visited and listed exactly once.
    for line in include_str!("input.txt").lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[..] {
            // Ignore:
            ["$", "ls"] | ["dir", _] => (),
            // Leave a directory:
            ["$", "cd", ".."] => result.push(parents.pop().unwrap()),
            // Enter a directory:
            ["$", "cd", _] => parents.push(0),
            // The size of a file contributes to all parent dirs:
            [size, _] => {
                let size: u64 = size.parse().unwrap();
                parents.iter_mut().for_each(|parent| *parent += size);
            }
            _ => panic!("{line}"),
        }
    }
    // Input ends deep in some directory. Simulate popping to the root.
    parents.reverse();
    result.append(&mut parents);

    result
}

#[test]
fn test_part1() {
    let solution: u64 = sizes().iter().filter(|&x| *x <= 100_000).sum();
    assert_eq!(1_307_902, solution);
}

#[test]
fn test_part2() {
    let sizes = sizes();
    let used = sizes.last().unwrap();
    let unused = 70_000_000 - used;
    let to_free = 30_000_000 - unused;
    let solution = sizes.iter().filter(|&x| *x >= to_free).min().unwrap();
    assert_eq!(7_068_748, *solution);
}
