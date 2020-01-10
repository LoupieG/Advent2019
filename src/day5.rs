use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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
    let mut p2Vec = numVec.to_vec();
    let code = get_diagnostic_code(&mut p1Vec, 1);
    let code2 = get_diagnostic_code2(&mut p2Vec, 5);
    println!("Day 5 part 1 = {}", code);
    println!("Day 5 part 2 = {}", code2);

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

fn get_diagnostic_code2(iVec: &mut Vec<i32>, input_instruction: i32) -> i32 {
    let mut index = 0;
    let mut result = -1;

    while index < iVec.len() {

        let operation = iVec[index] % 100;
        let mode1 = iVec[index] / 100 % 10;
        let mode2 = iVec[index] / 1000 % 10;
        let mode3 = iVec[index] / 10000 % 10;

        let location = match operation {
            1 | 2 | 7 | 8 => (match mode3 { 0 => iVec[index + 3], _ => iVec[iVec[index + 3] as usize] } as usize),
            3 | 4 | 5 | 6 => iVec[index + 1] as usize,
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
                println!("CODE {}", iVec[location]);
                result = iVec[location];
                break;
            },
            5 =>  {
                //let left = iVec[iVec[index + 1] as usize];
                let left = match mode1 { 0 => iVec[iVec[index + 1] as usize], _ => iVec[index + 1] };
                let right= match mode2 { 0 => iVec[iVec[index + 2] as usize], _ => iVec[index + 2] };
                if left != 0 {
                    //index = iVec[iVec[index + 2] as usize] as usize;
                    index = right as usize;
                } else {
                    index += 3;
                }
                iVec[location]
            },
            6 => {
                let left = match mode1 { 0 => iVec[iVec[index + 1] as usize], _ => iVec[index + 1] };
                let right= match mode2 { 0 => iVec[iVec[index + 2] as usize], _ => iVec[index + 2] };
                if left == 0 {
                    index = right as usize;
                } else {
                    index += 3;
                }
                iVec[location]
            },
            7 => {
                let left = match mode1 { 0 => iVec[iVec[index + 1] as usize], _ => iVec[index + 1] };
                let right = match mode2 { 0 => iVec[iVec[index + 2] as usize], _ => iVec[index + 2] };
                index += 4;
                if left < right {
                    1
                } else {
                    0
                }
            },
            8 => {
                let left = match mode1 { 0 => iVec[iVec[index + 1] as usize], _ => iVec[index + 1] };
                let right = match mode2 { 0 => iVec[iVec[index + 2] as usize], _ => iVec[index + 2] };
                index += 4;
                if left == right {
                    1
                } else {
                    0
                }
            },
            99 => break,
            _ => break,
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let istring = "3,9,8,9,10,9,4,9,99,-1,8".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 0), 0);
    }

    #[test]
    fn example_2() {
        let istring = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 0), 0);
    }

    #[test]
    fn example_3() {
        let istring = "3,3,1108,-1,8,3,4,3,99".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 8), 1);
    }

    #[test]
    fn example_4() {
        let istring = "3,9,7,9,10,9,4,9,99,-1,8".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 5), 1);
    }

    #[test]
    fn example_5() {
        let istring = "3,9,7,9,10,9,4,9,99,-1,8".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 9), 0);
    }

    #[test]
    fn example_6() {
        let istring = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 9), 1001);
    }

    #[test]
    fn example_7() {
        let istring = "3,3,1107,-1,8,3,4,3,99".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 3), 1);
    }

    #[test]
    fn example_8() {
        let istring = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1".to_string();
        let numVec: Vec<i32> = istring
            .lines()
            .next()
            .unwrap()
            .split(",").map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut p1Vec = numVec.to_vec();
        assert_eq!(get_diagnostic_code2(&mut p1Vec, 0), 0);
    }
}