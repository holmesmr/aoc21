use criterion::{BatchSize, Criterion};
use std::fs::File;
use std::io::BufReader;

pub fn bench_aoc<O, F>(c: &mut Criterion, name: &str, filename: &str, soln: F)
where
    F: Fn(BufReader<File>) -> O,
{
    c.bench_function(name, |b| {
        b.iter_batched(
            || BufReader::new(std::fs::File::open(filename).expect("file open failed")),
            |f| soln(f),
            BatchSize::PerIteration,
        )
    });
}
