advent_of_code::solution!(2);

struct IdGroup {
    start: u64,
    end: u64,
}

impl IdGroup {
    fn is_invalid(&mut self) -> bool {
        let len = IdGroup::get_num_digits_u64(self.start);
        if len % 2 == 1 || len == 0 {
            return false;
        } //id is odd and therefore not invalid or len is 0 and is not an ID
        let len = len / 2;
        let start_1 = self.start;
        let start_2: u64 = start_1.to_string().split_off(len).parse::<u64>().unwrap(); //ParseIntError { kind: Empty }
        if start_1 == start_2 {
            self.start += 1;
            return true;
        }
        true
    }
    pub fn get_num_digits_u64(mut n: u64) -> usize {
        if n == 0 {
            return 1; // Special case for 0, which has one digit
        }
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    }
}

/* # Part One
 * If all numbers are the same
 * if some sequence of digits repeated twice
 * leading 0's aren't an ID at all
 */

pub fn part_one(input: &str) -> Option<u64> {
    let mut split_input = input.split(",");
    let mut ids: Vec<IdGroup> = Vec::new();
    let mut output = 0;
    while let Some(id) = split_input.next() {
        let id = id.split_once("-").unwrap();
        let group = IdGroup {
            start: id.0.parse::<u64>().unwrap(),
            end: id.1.parse::<u64>().unwrap(),
        };
        ids.push(group);
    }

    for mut group in ids {
        while group.start < group.end + 1 {
            if IdGroup::is_invalid(&mut group) {
                output += 1
            }
        }
    }

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let output = 0;
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
