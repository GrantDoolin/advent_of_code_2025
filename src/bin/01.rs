advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut value = 50;
    let mut password: u64 = 0;
    for line in input.lines() {
        let cur_line = line;
        if cur_line.trim().contains("R") {
            let cur_line = &cur_line[1..];
            value = value + cur_line.parse::<i32>().unwrap().rem_euclid(100);
            if value > 99 {
                value = value - 100;
            }
            if value == 0 {
                password += 1;
            }
        } else {
            let cur_line = &cur_line[1..];
            value = value - cur_line.parse::<i32>().unwrap().rem_euclid(100);
            if value < 0 {
                value = value + 100;
            }
            if value == 0 {
                password += 1;
            }
        }
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut value = 50;
    let mut password: u64 = 0;
    for line in input.lines() {
        let cur_line = line;
        if cur_line.trim().contains("R") {
            let cur_line = &cur_line[1..];
            if cur_line.parse::<u64>().unwrap() > 99 {
                password = password + (cur_line.parse::<u64>().unwrap() / 100)
            }
            let pref_value = value;
            value = value + cur_line.parse::<i32>().unwrap().rem_euclid(100);
            if value == 0 {
                password += 1;
            } else if value > 99 {
                value = value - 100;
                if pref_value != 0 {
                    password += 1
                }
            }
        } else {
            let cur_line = &cur_line[1..];
            if cur_line.parse::<u64>().unwrap() > 99 {
                password = password + (cur_line.parse::<u64>().unwrap() / 100)
            }
            let pref_value = value;
            value = value - cur_line.parse::<i32>().unwrap().rem_euclid(100);
            if value == 0 {
                password += 1;
            } else if value < 0 {
                value = value + 100;
                if pref_value != 0 {
                    password += 1
                }
            }
        }
    }
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
