advent_of_code::solution!(2);

type Num = u64;

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = parse_ranges(input);
    let doubles = ranges.iter().map(|(lower, upper)| {
        find_doubles(*lower, *upper)
    }).collect::<Vec<_>>().concat();

    doubles.iter().sum::<u64>().try_into().ok()
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

fn parse_ranges(input: &str) -> Vec<(u64, u64)> {
    input.trim().split(",").filter(|part| !part.trim().is_empty()).map(|part| {
        let part = part.trim();
        let mut bounds = part.split("-");
        let start: u64 = bounds.next().unwrap().parse().unwrap();
        let end: u64 = bounds.next().unwrap().parse().unwrap();
        (start, end)
    }).collect() // Collect parsed ranges into a vector and return it
}

fn find_doubles(lower: u64, upper: u64) -> Vec<u64> {
    let mut doubles = Vec::new();
    for i in lower..=upper {
        if is_double(i) {
            doubles.push(i);
        }
    }
    doubles
}

fn is_double(n: u64) -> bool {
    //dbg!(n);
    let digits: Vec<char> = n.to_string().chars().collect();
    // if odd, not repeated
    if digits.len() % 2 != 0 {
        return false;
    }

    // check if the first half of the digits are repeated in the second half
    let len = digits.len();
    for i in 0..len / 2 {
        //dbg!(i, digits[i], digits[len/2 + i]);
        if digits[i] != digits[len/2 + i] {
            return false;
        }
    }
    true
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

    #[test]
    fn test_find_doubles() {
        let doubles = find_doubles(11, 22);
        assert_eq!(doubles, vec![11, 22]);
    }

    #[test]
    fn test_parse_ranges() {
        let input = "5-8,0-2,4-7";
        let ranges = parse_ranges(input);
        assert_eq!(ranges, vec![(5, 8), (0, 2), (4, 7)]);
    }

    #[test]
    fn test_is_double() {
        assert!(!is_double(1122));
        assert!(!is_double(1234));
        assert!(is_double(1111));
        assert!(is_double(1212));
    }
}
