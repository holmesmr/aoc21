use anyhow::{anyhow, bail, Context};
use std::str::FromStr;

use itertools::Itertools;

use crate::common::parse_input_as_value_list_anyhow;

/// A movement direction that can be commanded.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        Ok(match s {
            "forward" => Forward,
            "down" => Down,
            "up" => Up,
            s => bail!("Unrecognised direction: {}", s),
        })
    }
}

/// A command to move the submarine in a direction by a given number of units.
#[derive(Clone, Debug)]
pub struct MoveCommand(Direction, u32);

impl FromStr for MoveCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("malformed move command: no space"))?;

        let dir = dir
            .parse()
            .context("failed to parse direction part of command")?;
        let amt = amt
            .parse()
            .context("failed to parse amount part of command")?;

        Ok(MoveCommand(dir, amt))
    }
}

/// The submarine's position in the ocean.
///
/// In face of normal conventions, z is measured as units _below_ sea level.
#[derive(Clone, Default, Debug)]
struct Submarine {
    pos_x: i64,
    pos_z: i64,
    aim: i64,
}

impl Submarine {
    /// Perform a movement on the submarine.
    pub fn perform_simple(mut self, cmd: MoveCommand) -> Self {
        use Direction::*;

        // Destructure move into direction and distance
        let MoveCommand(dir, amt) = cmd;
        let amt = amt as i64;

        match dir {
            Up => self.pos_z -= amt,
            Down => self.pos_z += amt,
            Forward => self.pos_x += amt,
        }

        self
    }

    /// Perform a movement on the submarine.
    pub fn perform_aimed(mut self, cmd: MoveCommand) -> Self {
        use Direction::*;

        // Destructure move into direction and distance
        let MoveCommand(dir, amt) = cmd;
        let amt = amt as i64;

        match dir {
            Up => self.aim -= amt,
            Down => self.aim += amt,
            Forward => {
                self.pos_x += amt;
                self.pos_z += amt * self.aim;
            },
        }

        self
    }
}

/// Calculate the final position of the submarine after a series of movements.
pub fn solve_part1<R: std::io::BufRead>(input: R) -> anyhow::Result<i64> {
    let sub: Submarine = Default::default();

    let sub = parse_input_as_value_list_anyhow::<_, MoveCommand>(input)
        .fold_ok(sub, |sub, cmd| sub.perform_simple(cmd))?;

    Ok(sub.pos_x * sub.pos_z)
}

/// Count the number of increases over a 3-reading sliding window.
///
/// The increase is calculated between the sum of the last sliding window and the present.
///
/// The first window is not considered an increase.
pub fn solve_part2<R: std::io::BufRead>(input: R) -> anyhow::Result<i64> {
    let sub: Submarine = Default::default();

    let sub = parse_input_as_value_list_anyhow::<_, MoveCommand>(input)
        .fold_ok(sub, |sub, cmd| sub.perform_aimed(cmd))?;

    Ok(sub.pos_x * sub.pos_z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SAMPLE_INPUT: &[u8] = b"\
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn part1_sample_produces_expected_result() {
        let input = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part1(input).expect("error parsing input"), 150);
    }

    #[test]
    fn part2_sample_produces_expected_result() {
        let input = Cursor::new(SAMPLE_INPUT);

        assert_eq!(solve_part2(input).expect("error parsing input"), 900);
    }
}
