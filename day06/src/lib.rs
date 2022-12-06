#![no_std]
#![cfg(test)]
#![warn(clippy::all, clippy::pedantic, unsafe_code)]

fn is_marker(window: &[u8]) -> bool {
    let mut bits = 0;
    for byte in window {
        let previous_bits = bits;
        bits |= 1 << (byte - b'a');
        if bits == previous_bits {
            return false;
        }
    }
    true
}

fn first_marker(text: &str, window_size: usize) -> usize {
    let position = text.as_bytes().windows(window_size).position(is_marker);
    position.unwrap() + window_size
}

#[test]
fn test_part1() {
    let position = first_marker(include_str!("input.txt"), 4);
    assert_eq!(1_850, position);
}

#[test]
fn test_part2() {
    let position = first_marker(include_str!("input.txt"), 14);
    assert_eq!(2_823, position);
}
