fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();

    println!("{}", aoc21::day01::solve_part1(stdin.lock())?);

    Ok(())
}
