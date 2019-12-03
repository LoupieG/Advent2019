use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::borrow::Borrow;

pub fn day2() -> io::Result<()> {
    let file = File::open("./input/Day2.txt")?;
    let reader = BufReader::new(file);

    let mut numVec: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",").map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    calculate_vector(&mut numVec);
    println!("Day 2 part 1 = {}", numVec[0]);
    Ok(())

}

fn calculate_vector(iVec: &mut Vec<i32>) {
    for index in (0..iVec.len() - 1).step_by(4) {
        let loc = iVec[index + 3] as usize;
        if iVec[index].eq(1.borrow()) {
           iVec[loc] = iVec[iVec[index + 1] as usize] + iVec[iVec[index + 2] as usize];
        }
        else if iVec[index].eq(2.borrow()) {
            iVec[loc] = iVec[iVec[index + 1] as usize] * iVec[iVec[index + 2] as usize];
        }
        else if iVec[index].eq(99.borrow()) {
            break;
        }
    }
}