use anyhow::{Result, anyhow};
use aoc_2025::core::*;

#[derive(Debug)]
struct Safe {
    dial_state: usize,
    zero_count: usize,
    zero_crosses: usize,
}

#[derive(Debug)]
enum Direction {
    Left(usize),
    Right(usize),
}

impl Direction {
    fn parse(input: Vec<String>) -> Result<Vec<Direction>> {
        Ok(input
            .iter()
            .map(|line| {
                let (direction, count) = line.split_at(1);
                match direction {
                    "L" => Ok(Direction::Left(
                        count.parse().map_err(|_| anyhow!("Parse Error"))?,
                    )),
                    "R" => Ok(Direction::Right(
                        count.parse().map_err(|_| anyhow!("Parse Error"))?,
                    )),
                    _ => Err(anyhow!("Unexpected Input")),
                }
            })
            .collect::<Result<Vec<Direction>, anyhow::Error>>()?)
    }
}

impl Safe {
    pub fn new(dial_state: usize) -> Self {
        Self {
            dial_state,
            zero_count: 0,
            zero_crosses: 0,
        }
    }

    pub fn rotate(&mut self, direction: &Direction) {
        let result = match direction {
            Direction::Left(count) => self.dial_state as isize - *count as isize,
            Direction::Right(count) => (self.dial_state as isize + *count as isize),
        };

        let (wrapped_result, crosses) = self.wrap(result);

        self.dial_state = wrapped_result;
        self.zero_crosses += crosses;

        if self.dial_state == 0 {
            self.zero_count += 1;
        }
    }

    fn wrap(&self, input: isize) -> (usize, usize) {
        let wrapped = ((input % 100) + 100) % 100;
        let idial_state = self.dial_state as isize;

        let crosses = if input < idial_state {
            (idial_state - 1).div_euclid(100) - (input).div_euclid(100)
        } else {
            (input - 1) / 100 - idial_state / 100
        };

        (wrapped as usize, crosses as usize)
    }
}

fn problem_one(input: Vec<String>) -> Result<()> {
    let mut safe = Safe::new(50);
    let parsed = Direction::parse(input)?;

    parsed.iter().for_each(|direction| safe.rotate(direction));

    println!("{}", safe.zero_count);

    Ok(())
}

fn problem_two(input: Vec<String>) -> Result<()> {
    let mut safe = Safe::new(50);
    let parsed = Direction::parse(input)?;

    parsed.iter().for_each(|direction| safe.rotate(direction));

    println!("{}", safe.zero_crosses + safe.zero_count);

    Ok(())
}

fn main() -> Result<()> {
    let input = get_lines("day-1")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_1_tests {
    use crate::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_problem_one() -> Result<()> {
        let result = Direction::parse(INPUT.lines().map(String::from).collect())?;
        let mut safe = Safe::new(50);

        result.iter().for_each(|direction| safe.rotate(direction));
        assert_eq!(safe.zero_count, 3);
        Ok(())
    }

    #[test]
    fn test_problem_two() -> Result<()> {
        let result = Direction::parse(INPUT.lines().map(String::from).collect())?;
        let mut safe = Safe::new(50);

        result.iter().for_each(|direction| safe.rotate(direction));
        println!("{:?}", safe);
        assert_eq!(safe.zero_crosses + safe.zero_count, 6);
        Ok(())
    }

    #[test]
    fn test_problem_multi_wrap() -> Result<()> {
        let result = Direction::parse(vec![String::from("R1000")])?;
        let mut safe = Safe::new(50);

        result.iter().for_each(|direction| safe.rotate(direction));
        assert_eq!(safe.zero_crosses, 10);
        Ok(())
    }

    #[test]
    fn test_problem_left_zero() -> Result<()> {
        let result = Direction::parse(vec![String::from("L22")])?;
        let mut safe = Safe::new(22);

        result.iter().for_each(|direction| safe.rotate(direction));
        assert_eq!(safe.zero_crosses + safe.zero_count, 1);
        Ok(())
    }

    #[test]
    fn test_problem_right_zero() -> Result<()> {
        let result = Direction::parse(vec![String::from("R199")])?;
        let mut safe = Safe::new(1);

        result.iter().for_each(|direction| safe.rotate(direction));
        assert_eq!(safe.zero_crosses + safe.zero_count, 2);
        Ok(())
    }

    #[test]
    fn test_problem_one_input() -> Result<()> {
        let input = get_lines("day-1")?;
        let mut safe = Safe::new(50);
        let parsed = Direction::parse(input)?;

        parsed.iter().for_each(|direction| safe.rotate(direction));

        assert_eq!(safe.zero_count, 1043);

        Ok(())
    }

    #[test]
    fn test_problem_two_input() -> Result<()> {
        let input = get_lines("day-1")?;
        let mut safe = Safe::new(50);
        let parsed = Direction::parse(input)?;

        parsed.iter().for_each(|direction| safe.rotate(direction));

        assert_eq!(safe.zero_crosses + safe.zero_count, 5963);

        Ok(())
    }
}
