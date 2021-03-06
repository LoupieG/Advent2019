use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::borrow::Borrow;

pub fn day2() -> io::Result<()> {
    let file = File::open("./input/Day2.txt")?;
    let reader = BufReader::new(file);

    let numVec: Vec<i32> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",").map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut p1Vec = numVec.to_vec();
    calculate_vector(&mut p1Vec);
    println!("Day 2 part 1 = {}", p1Vec[0]);

    for x in 0..99 {
        for y in 0..99 {
           let mut p2Vec = numVec.to_vec();
           p2Vec[1] = x;
           p2Vec[2] = y;
           calculate_vector(&mut p2Vec);
            if p2Vec[0].eq(19690720.borrow()) {
               println!("Day 2 part 2 total {}", (p2Vec[1] * 100) + p2Vec[2]);
               break;
           }
        }
    }

    Ok(())

}

fn calculate_vector(iVec: &mut Vec<i32>) {
    for index in (0..iVec.len() - 1).step_by(4) {
        let loc = iVec[index + 3] as usize;
        iVec[loc] = match iVec[index] {
            1 => iVec[iVec[index + 1] as usize] + iVec[iVec[index + 2] as usize],
            2 => iVec[iVec[index + 1] as usize] * iVec[iVec[index + 2] as usize],
            99 => break,
            _ => break,
        }
    }
}