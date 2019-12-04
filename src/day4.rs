
pub fn day4() {

    let mut passCount = 0;
    let mut passCount2 = 0;

    for n in 272091..815432 {
        let svec: Vec<char> = n.to_string().chars().collect();
        if is_valid(&svec) {
            passCount += 1;
        }
        if is_valid2(&svec) {
            passCount2 += 1;
        }
    }
    println!("Day 4 pass count part 1 {}", passCount);
    println!("Day 4 pass count part 2 {}", passCount2);
}

fn is_valid(pass: &Vec<char>) -> bool {
    let mut result = true;

    let mut hasDouble = false;

    for x in 1..pass.len() {
        if pass[x - 1] > pass[x] {
            result = false;
            break;
        }

        if pass[x - 1] == pass[x] {
            hasDouble = true;
        }
    }

    (result && hasDouble)
}

fn is_valid2(pass: &Vec<char>) -> bool {
    let mut result = true;

    let mut doubleCount = 0;

    let mut counter = 0;

    while counter < pass.len() - 1 {
        if pass[counter] > pass[counter + 1] {
            result = false;
            break;
        }

        if pass[counter] == pass[counter + 1] {
            doubleCount += 1;
        }

        if (counter + 2) < pass.len() && pass[counter] == pass[counter + 1] {
            if pass[counter] == pass[counter + 1] && pass[counter + 1] == pass[counter + 2] {
                let mut icount = counter + 1;
                doubleCount -= 1;
                while icount < pass.len() - 1 {
                     if pass[counter] == pass[icount + 1] { icount += 1; }
                     else { break; }
                }
                counter = icount;
            } else {counter += 1;}
        } else {counter += 1; }
    }

    (result && doubleCount > 0)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(is_valid(&"111111".chars().collect()), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(is_valid(&"223450".chars().collect()), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(is_valid(&"111122".chars().collect()), true);
    }

    #[test]
    fn example_4() {
        assert_eq!(is_valid2(&"112233".chars().collect()), true);
    }

    #[test]
    fn example_5() {
        assert_eq!(is_valid2(&"123444".chars().collect()), false);
    }

    #[test]
    fn example_6() {
        assert_eq!(is_valid2(&"111122".chars().collect()), true);
    }

    #[test]
    fn example_7() {
        assert_eq!(is_valid2(&"799998".chars().collect()), false);
    }

    #[test]
    fn example_8() {
        assert_eq!(is_valid2(&"788889".chars().collect()), false);
    }

    #[test]
    fn example_9() {
        assert_eq!(is_valid2(&"789999".chars().collect()), false);
    }

    #[test]
    fn example_10() {
        assert_eq!(is_valid2(&"779998".chars().collect()), false);
    }
}