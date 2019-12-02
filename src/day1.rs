use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn day1() -> io::Result<()> {
    let file = File::open("/home/kmurray/src/rust/Advent2019/src/Day1.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    let mut module_fuel = 0;

    for line in reader.lines() {
        let fuel = line?.trim().parse::<i32>().unwrap();
        total_fuel += get_fuel_load(fuel);
        module_fuel += get_wishful_load(fuel);
    }
    println!("total fuel {}", total_fuel);
    println!("module fuel {}", module_fuel);

    Ok(())

}

fn get_fuel_load(fuel: i32) -> i32 {
    (fuel / 3) - 2
}

fn get_wishful_load(fuel: i32) -> i32 {
    let mut mod_sum = 0;

    let mut next_fuel = fuel;
    loop {
        let calc_fuel = get_fuel_load(next_fuel);

        if calc_fuel > 0 {
            next_fuel = calc_fuel;
            mod_sum += next_fuel;
        }
        else {
            break;
        }
    }
    mod_sum
}