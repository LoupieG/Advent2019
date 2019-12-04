//use std::fs::File;
use std::io::{self, prelude::*, BufReader};
//use std::collections::HashSet;

pub fn day3() -> io::Result<()> {
    /*

    This is going to take me some time to grok. I want to do this with a hashset so I can utilize
    the intersect method. This will not be a quick attack on the problem since I want to do
    everything in rust and this is heading into some deep territory that I haven't explored yet.

    let file = File::open("./input/Day3Sample.txt")?;
    let mut reader = BufReader::new(file);

    let mut sinput = String::new();
    let num_bytes = reader.read_line(&mut sinput);

    println!("input = {}", sinput);

    let mut mymap = sinput
        .lines()
        .map(|line| {
            let text = line.split(',');
            let mut wire = Vec::new();

            for dir in text {
                let dir = match dir.chars().nth(0).unwrap() {
                    'U' => [dir[1..].parse().unwrap(), 0],
                    'D' => [(0 - dir[1..].parse().unwrap()), 0],
                    'L' => [0, (0 - dir[1..].parse().unwrap())],
                    'R' => [0, dir[1..].parse().unwrap()],
                    _ => panic!("Bad command"),
                };
                wire.push(dir);
            }
            wire
        });

    let grids = [mymap.next().unwrap(), mymap.next().unwrap()];


    //let intersections = grids

    println!("Vec1 {:?}", grids[0]);
   // println!("Vec2 {:?}", vec2);
*/
    Ok(())
}

//fn lay_wire(input: &str) -> (HashSet<[i32; 2]>, Vec<[i32; 2]>) {
//    let mut result: HashSet<(i32, i32), RandomState> = HashSet::new();
//
//}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_fuel_load(12), 2);
    }
}
*/