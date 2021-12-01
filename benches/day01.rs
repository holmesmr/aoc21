use aoc21::day01::{solve_part1, solve_part2};
use criterion::{criterion_group, criterion_main, Criterion};

mod common;

use crate::common::bench_aoc;

pub fn part1_benchmark(c: &mut Criterion) {
    bench_aoc(c, "day01 part1", "inputs/day01.txt", solve_part1);
}

pub fn part2_benchmark(c: &mut Criterion) {
    bench_aoc(c, "day01 part2", "inputs/day01.txt", solve_part2);
}

criterion_group!(day01, part1_benchmark, part2_benchmark);
criterion_main!(day01);
