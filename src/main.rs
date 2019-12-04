#![allow(non_snake_case)]

use std::io::{self};

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> io::Result<()> {
    day1::day1()?;
    day2::day2()?;
    day3::day3()?;
    day4::day4();

    Ok(())

}
