use std::process::Output;

advent_of_code::solution!(2);

struct id_group {
    start: u64,
    end: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut split_input = input.split(",");
    let mut ids: Vec<id_group> = Vec::new();
    while let Some(id) = split_input.next() {
        let id = id.split_once("-").unwrap();
        let group = id_group {
            start: id.0.parse::<u64>().unwrap(),
            end: id.1.parse::<u64>().unwrap(),
        };
        ids.push(group);
    }

    for group in ids {
        if group.start != group.end {}
    }
    let output = 0;
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
