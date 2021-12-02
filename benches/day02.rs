use aoc21::day02::{solve_part1, solve_part2};
use criterion::{criterion_group, criterion_main, Criterion};

mod common;

use crate::common::bench_aoc;

pub fn part1_benchmark(c: &mut Criterion) {
    bench_aoc(c, "day02 part1", "inputs/day02.txt", solve_part1);
}

pub fn part2_benchmark(c: &mut Criterion) {
    bench_aoc(c, "day02 part2", "inputs/day02.txt", solve_part2);
}

criterion_group!(day02, part1_benchmark, part2_benchmark);
criterion_main!(day02);
