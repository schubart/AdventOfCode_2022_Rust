#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

fn values() -> Vec<isize> {
    let mut values = Vec::new();
    let mut x = 1;
    for line in include_str!("input.txt").lines() {
        values.push(x);
        if line != "noop" {
            values.push(x);
            let delta: isize = line.split_at(5).1.parse().unwrap();
            x += delta;
        }
    }
    values
}

#[test]
fn test_part1() {
    let result: isize = values()
        .iter()
        .enumerate()
        .map(|(i, &x)| (isize::try_from(i).unwrap() + 1) * x)
        .skip(19)
        .step_by(40)
        .sum();
    assert_eq!(14_320, result);
}

#[test]
fn test_part2() {
    let result: String = values()
        .iter()
        .zip((0..40).cycle())
        .map(|(value, position)| {
            if (value - position).abs() <= 1 {
                '#'
            } else {
                ' '
            }
        })
        .collect();

    assert_eq!(
        "###   ##  ###  ###  #  #  ##  ###    ## \
         #  # #  # #  # #  # # #  #  # #  #    # \
         #  # #    #  # ###  ##   #  # #  #    # \
         ###  #    ###  #  # # #  #### ###     # \
         #    #  # #    #  # # #  #  # #    #  # \
         #     ##  #    ###  #  # #  # #     ##  ",
        result
    );
}
