use anyhow;

mod day1;
mod day2;

fn main() -> anyhow::Result<()> {
    dbg!(day1::day_one_solution_one());
    dbg!(day1::day_one_solution_two());
    dbg!(day2::day_two_solution_one()?);
    dbg!(day2::day_two_solution_two()?);
    Ok(())
}
