use crate::common::parse_input_as_value_list;
use itertools::Itertools;

/// Count the number of increases in a list of height readings.
///
/// The first item is not considered an increase.
pub fn solve_part1<R: std::io::BufRead>(input: R) -> anyhow::Result<i32> {
    // Counting from -1 means the first increase won't be counted, and all
    // inputs are positive integers > 0, so the first input will always
    // increase the count.
    let mut count = -1;
    let mut last = 0;
    for i in parse_input_as_value_list::<_, i32>(input) {
        let i = i?;

        if i > last {
            count += 1;
        }

        last = i;
    }

    Ok(count)
}

/// Count the number of increases over a 3-reading sliding window.
///
/// The increase is calculated between the sum of the last sliding window and the present.
///
/// The first window is not considered an increase.
pub fn solve_part2<R: std::io::BufRead>(input: R) -> anyhow::Result<i32> {
    // As before, count from -1 for simplicity
    let mut count = -1;
    let mut last = 0;

    let readings = parse_input_as_value_list::<_, i32>(input);

    for (a, b, c) in readings.filter_map(|ent| ent.ok()).tuple_windows() {
        let sum = a + b + c;

        if sum > last {
            count += 1;
        }

        last = sum;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE_INPUT: &[u8] = b"\
199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn part1_sample_produces_expected_result() {
        let input = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(input).expect("error parsing input"), 7);
    }

    #[test]
    fn part2_sample_produces_expected_result() {
        let input = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(input).expect("error parsing input"), 5);
    }
}
