use anyhow;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> anyhow::Result<()> {
    dbg!(day1::day_one_solution_one());
    dbg!(day1::day_one_solution_two());
    dbg!(day2::day_two_solution_one()?);
    dbg!(day2::day_two_solution_two()?);
    dbg!(day3::day_three_solution_one());
    dbg!(day3::day_three_solution_two());
    dbg!(day4::day_four_solution_one());
    dbg!(day4::day_four_solution_two());
    dbg!(day5::day_five_solution_one()?);
    dbg!(day5::day_five_solution_two()?);
    dbg!(day6::day_six_solution_one());
    dbg!(day6::day_six_solution_two());
    Ok(())
}
