#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

fn snafu_to_int(snafu: &str) -> isize {
    let mut result = 0;
    for c in snafu.chars() {
        result *= 5;
        result += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            other => panic!("{other}"),
        };
    }
    result
}

fn int_to_snafu(mut int: isize) -> String {
    if int == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    while int != 0 {
        result.push(match int % 5 {
            2 => '2',
            1 => '1',
            0 => '0',
            3 => '=',
            4 => '-',
            other => panic!("{other}"),
        });
        int += 2;
        int /= 5;
    }

    result.chars().rev().collect()
}

#[test]
fn test_part1() {
    let sum = include_str!("input.txt").lines().map(snafu_to_int).sum();
    let result = int_to_snafu(sum);

    assert_eq!("20=02=120-=-2110-0=1", result);
}
