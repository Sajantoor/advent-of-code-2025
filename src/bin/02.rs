use std::collections::HashMap;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    // split the input by ','
    let ranges = input.trim().split(',');

    for range in ranges {
        let numbers: Vec<&str> = range.split('-').collect();
        let start: u64 = numbers.get(0).unwrap().parse().unwrap();
        let end: u64 = numbers.get(1).unwrap().parse().unwrap();

        // an invalid number has a pattern repeated twice in the range
        // invalid numbers cannot start with 0
        for i in start..(end + 1) {
            let str: String = i.to_string();

            if str.len() % 2 == 0 {
                let middle = str.len() / 2;
                let start = &str[0..middle];
                let end = &str[middle..str.len()];

                if start.eq(end) {
                    result += i;
                }
            }
        }
    }

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    return None;
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
        assert_eq!(result, Some(4174379265));
    }
}
