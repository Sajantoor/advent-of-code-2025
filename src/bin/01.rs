advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial_value: i32 = 50;
    let mut result = 0;

    for line in input.lines() {
        let start = line.get(..1);
        let number = line.get(1..);
        let number: i32 = number.unwrap().parse().unwrap();

        if let Some("L") = start {
            let result: i32 = (dial_value - number) % 100;
            if result < 0 {
                dial_value = 100 + result;
            } else {
                dial_value = result;
            }
        } else if let Some("R") = start {
            let result = dial_value + number;
            dial_value = result % 100;
        }

        if dial_value == 0 {
            result += 1;
        }
    }

    return result.try_into().unwrap();
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
