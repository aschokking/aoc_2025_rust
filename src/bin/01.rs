advent_of_code::solution!(1);

type Instr = (char, u32);

fn prepare_input(input: &str) -> Vec<Instr> {
    input.lines().map(|line| {
        let (action, value) = line.split_at(1);
        (action.chars().next().unwrap(), value.parse().unwrap())
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions: Vec<Instr> = prepare_input(input);

    let values: Vec<i32> = instructions.iter().fold(vec![50], |acc, (action, value)| {
        let mut new_acc = acc.clone();
        let delta: i32 = match action {
            'L' => -(*value as i32),
            'R' => *value as i32,
            _ => // throw error
                panic!("Unknown action"),
        };
        new_acc.push((new_acc.last().unwrap() + delta) % 100);
        new_acc
    });

    // count zeroes
    values.iter().filter(|&&x| x == 0).count().try_into().ok()
}

#[derive(Debug, PartialEq, Eq)]
struct Acc {
    count: u32,
    value: u32,
}

pub fn part_two(_input: &str) -> Option<u64> {
    let instructions = prepare_input(_input);

    let result: Acc = instructions.iter().fold(Acc { count: 0, value: 50 }, |acc, (action, value)| {
        let result = apply_instruction(acc.value, &( *action, *value ));
        Acc { count: acc.count + result.count, value: result.value }
    });

    Some(result.count as u64)
}

fn apply_instruction(position: u32, instruction: &Instr) -> Acc {
    let (action, value) = instruction;
    let delta: i32 = match action {
        'L' => -(*value as i32),
        'R' => *value as i32,
        _ => // throw error
            panic!("Unknown action"),
    };
    let raw_new_position = position as i32 + delta;
    let new_position: u32 = (raw_new_position).rem_euclid(100) as u32;
    let wrap_count = match raw_new_position {
        x if x > 0 => x.div_euclid(100) as u32,
        x if x < 0 && new_position == 0 => (x.div_euclid(100).abs() + 1) as u32,
        x if x < 0 && position != 0 => (x.div_euclid(100).abs()) as u32,
        x if x < 0 => (x.div_euclid(100).abs() - 1) as u32,
        x if x == 0 => 1,
        _ => unreachable!(),
        };
    Acc { count: wrap_count, value: new_position }
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

    #[test]
    fn test_apply_instruction() {
        assert_eq!(apply_instruction(50, &('L', 10)), Acc { count: 0, value: 40 });
        assert_eq!(apply_instruction(50, &('R', 10)), Acc { count: 0, value: 60 });
        assert_eq!(apply_instruction(50, &('L', 60)), Acc { count: 1, value: 90 });
        assert_eq!(apply_instruction(50, &('R', 60)), Acc { count: 1, value: 10 });

        assert_eq!(apply_instruction(50, &('R', 150)), Acc { count: 2, value: 0 });
        assert_eq!(apply_instruction(50, &('L', 149)), Acc { count: 1, value: 1 });
        assert_eq!(apply_instruction(50, &('L', 151)), Acc { count: 2, value: 99 });
        assert_eq!(apply_instruction(50, &('L', 150)), Acc { count: 2, value: 0 });
    }

    #[test]
    fn test_math() {
        assert_eq!(50i32.div_euclid(100), 0);
        assert_eq!(100i32.div_euclid(100), 1);
        assert_eq!(101i32.div_euclid(100), 1);
        assert_eq!(200i32.div_euclid(100), 2);

        assert_eq!(0i32.div_euclid(100), 0);
        assert_eq!(-100i32.div_euclid(100), -1);
        assert_eq!((-100i32).div_euclid(100).abs(), 1);
        assert_eq!((-18i32).div_euclid(100).abs(), 1);
        assert_eq!((-99i32).div_euclid(100).abs(), 1);
        assert_eq!((-101i32).div_euclid(100).abs(), 2);

        assert_eq!((-1i32).rem_euclid(100), 99);


    }
}
