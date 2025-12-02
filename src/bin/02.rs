use std::process::id;

advent_of_code::solution!(2);

struct IdGroup {
    start: u64,
    end: u64,
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
            let id = group.start;
            let len = get_num_digits_u64(id);
            if len % 2 == 0 {
                let id_end = id
                    .to_string()
                    .drain(len / 2..len)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                let id_begin = id
                    .to_string()
                    .drain(0..len / 2)
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                if id_begin.eq(&id_end) {
                    //split id and see if part 1 is == to part 2
                    output += id;
                    group.start += 1;
                } else {
                    group.start += 1;
                }
            }
            //need to check for odd numbers before id_end and id_begin creation
            else if len % 2 == 1 {
                //if ID has odd number of digits
                group.start += 1;
            }
        }
    }

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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

/*
*   println!("group in ids");
'group_check: while group.start < group.end + 1 {
    println!("before len check");
    let len = get_num_digits_u64(group.start);
    if len % 2 == 1 {
        println!("did we get here");
        group.start += 1;
        continue 'group_check;
    }
    let len = len / 2;
    let start_1 = group.start;
    let start_2: u64 = start_1.to_string().split_off(len).parse::<u64>().unwrap();
    if start_1 == start_2 {
        group.start += 1;
        output += 1;
        println!("AHH!");
    } else {
        output += 1;
        group.start += 1;
    }
}
*/
