use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn day1() -> io::Result<()> {
    let file = File::open("./input/Day1.txt")?;
    let reader = BufReader::new(file);

    let mut total_fuel = 0;
    let mut module_fuel = 0;

    for line in reader.lines() {
        let fuel = line?.trim().parse::<i32>().unwrap();
        let fuel_load = get_fuel_load(fuel);

        total_fuel += fuel_load;
        module_fuel += get_wishful_load(fuel_load);
    }
    println!("Day 1 total fuel {}", total_fuel);
    println!("Day 1 module fuel {}", module_fuel);

    Ok(())

}

fn get_fuel_load(fuel: i32) -> i32 {
    (fuel / 3) - 2
}

fn get_wishful_load(fuel_load: i32) -> i32 {
    let mut mod_sum = fuel_load;

    let mut next_fuel = fuel_load;
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

#[cfg(test)]
mod tests {
    use crate::day1::get_fuel_load;
    use crate::day1::get_wishful_load;

    #[test]
    fn example_1() {
        assert_eq!(get_fuel_load(12), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(get_fuel_load(14), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(get_fuel_load(1969), 654);
    }

    #[test]
    fn example_4() {
        assert_eq!(get_fuel_load(100756), 33583);
    }

    #[test]
    fn example_5() {
        assert_eq!(get_wishful_load(654), 966);
    }

    #[test]
    fn example_6() {
        assert_eq!(get_wishful_load(33583), 50346);
    }

}