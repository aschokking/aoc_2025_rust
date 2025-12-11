advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    input.lines().map(|line| find_largest_joltage(line)).sum::<u64>().try_into().ok()
}

fn find_largest_joltage(input: &str) -> u64 {
    let digits: Vec<char> = input.chars().collect();
    let mut largest = 0u64;

    // Try all pairs of digits maintaining their order
    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let two_digit = format!("{}{}", digits[i], digits[j]).parse::<u64>().unwrap_or(0);
            if two_digit > largest {
                largest = two_digit;
            }
        }
    }

    largest
}

pub fn part_two(input: &str) -> Option<u64> {
    input.lines().map(|line| find_largest_joltage2(line)).sum::<u64>().try_into().ok()
}

fn find_largest_joltage2(input: &str) -> u64 {
    let digits: Vec<char> = input.chars().collect();

    if digits.len() < 12 {
        return 0;
    }

    // Create a vector of (digit, original_index) pairs
    let mut indexed_digits: Vec<(char, usize)> = digits.iter().enumerate().map(|(i, &d)| (d, i)).collect();

    // Sort by digit value (descending) to find the 12 largest
    indexed_digits.sort_by(|a, b| b.0.cmp(&a.0));

    // Take the 12 largest and sort them descending by digit value for the final number
    let largest_12: Vec<char> = indexed_digits.iter().take(12).map(|(d, _)| *d).collect();

    // Build the number from the 12 largest digits in descending order
    let number_str: String = largest_12.iter().collect();
    number_str.parse::<u64>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_find_largest_joltage() {
        assert_eq!(find_largest_joltage("987654321111111"), 98);
    }
}
