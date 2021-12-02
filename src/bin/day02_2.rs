fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();

    println!("{}", aoc21::day02::solve_part2(stdin.lock())?);

    Ok(())
}
