use advent_of_code::ALL_DIRECTIONS_8;
use ndarray::Array2;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let map: Array2<char> = advent_of_code::parse_input(input);
    let valid_rolls = find_valid_rolls(&map).len();

    Some(valid_rolls as u64)
}

fn find_valid_rolls(map: &Array2<char>) -> Vec<(usize, usize)> {
    let mut valid_rolls: Vec<(usize, usize)> = Vec::new();
    // iterate over the index of each coordinate in the map array
    for (i, row) in map.axis_iter(ndarray::Axis(0)).enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value != '@' {
                continue;
            }

            let mut num_rolls = 0;
            // check the coordiantes around this one
            for (di, dj) in ALL_DIRECTIONS_8.iter() {
                let (ci, cj) = (i as i32 + di, j as i32 + dj);
                if ci < 0 || ci >= map.shape()[0] as i32 || cj < 0 || cj >= map.shape()[1] as i32 {
                    // skip this coordinate if it is out of bounds
                    continue;
                }

                if map[[ci as usize, cj as usize]] == '@' {
                    num_rolls += 1;
                }
            }

            if num_rolls <= 3 {
                valid_rolls.push((i, j));
            }

            // dbg!(i, j, value, num_rolls, num_valid);
        }
    }

    valid_rolls
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: Array2<char> = advent_of_code::parse_input(input);
    let mut total = 0;
    // loop until valid_rolls is empty
    loop {
        let valid_rolls = find_valid_rolls(&map);
        if valid_rolls.is_empty() {
            break;
        }
        total += valid_rolls.len() as u64;

        // remove all valid rolls from the map
        for (i, j) in valid_rolls {
            map[[i, j]] = 'x';
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
