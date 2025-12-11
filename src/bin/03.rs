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

#[derive(Debug, PartialEq, Eq)]
struct IndexedDigit {
    digit: char,
    index: usize,
}

fn find_largest_joltage2(input: &str) -> u64 {
    let digits: Vec<char> = input.chars().collect();

    if digits.len() < 12 {
        return 0;
    }

    // Create a vector of IndexedDigit structs
    let mut indexed_digits: Vec<IndexedDigit> = digits
        .iter()
        .enumerate()
        .map(|(i, &d)| IndexedDigit { digit: d, index: i })
        .collect();

    // Sort by digit value (descending), then by original index (ascending) as tiebreaker
    indexed_digits.sort_by(|a, b| {
        match b.digit.cmp(&a.digit) {
            std::cmp::Ordering::Equal => a.index.cmp(&b.index),
            other => other,
        }
    });
    // dbg!(&indexed_digits);
    let mut final_number = String::new();
    while final_number.len() < 12 {
        // find the biggest digit whose index is larger than input.len() - final_number.len() - 1
        let threshold_index = digits.len() - (12) + final_number.len();
        if let Some(pos) = indexed_digits.iter().position(|d| d.index <= threshold_index) {
            final_number.push(indexed_digits[pos].digit);
            // Extract the index before retain to avoid borrow conflict
            let selected_index = indexed_digits[pos].index;
            // remove everything with an index less than or equal to this digit's index
            indexed_digits.retain(|d| d.index > selected_index);
        } else {
            break;
        }
    }
    final_number.parse::<u64>().unwrap_or(0)
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
    fn test_find_largest_joltage2() {
        assert_eq!(find_largest_joltage2("987654321111111"), 987654321111);
        assert_eq!(find_largest_joltage2("811111111111119"), 811111111119);
        assert_eq!(find_largest_joltage2("234234234234278"), 434234234278);
        assert_eq!(find_largest_joltage2("818181911112111"), 888911112111);

    }


    #[test]
    fn test_find_largest_joltage() {
        assert_eq!(find_largest_joltage("987654321111111"), 98);
    }
}
