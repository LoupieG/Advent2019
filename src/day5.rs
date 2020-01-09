use std::fs::File;
use std::io::{self, prelude::*, BufReader};
//use std::borrow::Borrow;

pub fn day5() -> io::Result<()> {
    let file = File::open("./input/Day5.txt")?;
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
    let code = get_diagnostic_code(&mut p1Vec, 1);
    println!("Day 5 part 1 = {}", code);

    //for x in 0..99 {
    //    for y in 0..99 {
    //        let mut p2Vec = numVec.to_vec();
    //        p2Vec[1] = x;
    //        p2Vec[2] = y;
    //        calculate_vector(&mut p2Vec);
    //        if p2Vec[0].eq(19690720.borrow()) {
    //            println!("Day 2 part 2 total {}", (p2Vec[1] * 100) + p2Vec[2]);
    //            break;
    //        }
    //    }
    //}

    Ok(())

}

fn get_diagnostic_code(iVec: &mut Vec<i32>, input_instruction: i32) -> i32 {
    let mut index = 0;
    while index < iVec.len() {

        let operation = iVec[index] % 100;
        let mode1 = iVec[index] / 100 % 10;
        let mode2 = iVec[index] / 1000 % 10;
        let mode3 = iVec[index] / 10000 % 10;

        let location = match operation {
            1 | 2 => (match mode3 { 0 => iVec[index + 3], _ => iVec[iVec[index + 3] as usize] } as usize),
            3 | 4 => iVec[index + 1] as usize,
            99 => { println!("Diagnostic Code = {}", iVec[iVec[index - 1] as usize]); break },
            _ => panic!("Invalid operation {}", iVec[index]),
        };

        iVec[location] = match operation {
            1 => {
                let left = match mode1 { 0 => iVec[iVec[index + 1] as usize], _ => iVec[index + 1] };
                let right = match mode2 { 0 => iVec[iVec[index + 2] as usize], _ => iVec[index + 2] };
                index += 4;
                left + right
            },
            2 => {
                let left = match mode1 { 0 => iVec[iVec[index + 1] as usize], _ => iVec[index + 1] };
                let right = match mode2 { 0 => iVec[iVec[index + 2] as usize], _ => iVec[index + 2] };
                index += 4;
                left * right
            },
            3 => {
                index += 2;
                input_instruction // save to position
            },
            4 => {
                index += 2;
                iVec[location]
            },
            99 => break,
            _ => break,
        };
    }

    iVec[iVec[index - 1] as usize]

}
