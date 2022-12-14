#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

fn solve(reverse: bool) -> String {
    let mut lines = include_str!("input.txt").lines();

    // Parse stacks.
    let mut stacks = Vec::new();
    for line in &mut lines {
        // Skip line that contains stack numbers.
        if !line.contains('[') {
            break;
        }

        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            // Create stacks as needed.
            if stacks.len() <= i {
                stacks.push(Vec::new());
            }
            // Push to stack if there is an item.
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    // Stacks are upside down after parsing, reverse.
    stacks.iter_mut().for_each(|stack| stack.reverse());

    // Skip empty line.
    lines.next();

    // Parse and execute move instructions.
    for line in lines {
        let mut split = line.split(' ');
        let count: usize = split.nth(1).unwrap().parse().unwrap();
        let from: usize = split.nth(1).unwrap().parse().unwrap();
        let to: usize = split.nth(1).unwrap().parse().unwrap();

        // Fill up crane from source stack.
        let source = &mut stacks[from - 1];
        let mut crane = source.split_off(source.len() - count);

        // Reverse items or don't, depending on which model is used.
        if reverse {
            crane.reverse();
        }

        // Move items from crane to target stack.
        stacks[to - 1].append(&mut crane);
    }

    // Collect tops of stacks.
    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

#[test]
fn test_part1() {
    // CrateMover 9000: Moves items one by one, thereby reversing their order.
    let result = solve(true);
    assert_eq!("HBTMTBSDC", result);
}

#[test]
fn test_part2() {
    // CrateMover 9001: Moves items in one go, thereby maintaining their order.
    let result = solve(false);
    assert_eq!("PQTJRSHWS", result);
}
