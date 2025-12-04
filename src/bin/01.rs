advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    return helper(input, false);
}

fn helper(input: &str, should_count_touches: bool) -> Option<u64> {
    let mut dial_value: i32 = 50;
    let mut result: u64 = 0;

    for line in input.lines() {
        let direction = line.get(..1);
        let number = line.get(1..);
        let mut number: i32 = number.unwrap().parse().unwrap();

        // if number is greater than 99, then we're doing a full rotation, count the number of rotations and result += something
        if number > 99 {
            if should_count_touches {
                let number_of_rotations: u64 = (number / 100).try_into().unwrap();
                result += number_of_rotations;
            }
            number = number % 100;
        }

        if let Some("L") = direction {
            let diff = dial_value - number;

            // If we're going left, if the difference dial_value - number < 0, then result++
            if diff < 0 {
                // if we're at 0, then diff is going to be negative so make sure current dial value is not 0 when updating the count
                if should_count_touches && dial_value != 0 {
                    result += 1;
                }
                dial_value = 100 + diff;
            } else {
                dial_value = diff;
            }
        }

        if let Some("R") = direction {
            let sum = dial_value + number;

            // if we're going right, if the sum of dial_value + number > 99, then we're crossing 0, so result++
            if sum > 99 {
                dial_value = sum % 100;
                if should_count_touches && dial_value != 0 {
                    result += 1;
                }
                // else, we aren't crossing 0
            } else {
                dial_value = sum;
            }
        }

        // if we end up at 0 then result++
        if dial_value == 0 {
            result += 1;
        }
    }

    return result.try_into().unwrap();
}

pub fn part_two(input: &str) -> Option<u64> {
    return helper(input, true);
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
