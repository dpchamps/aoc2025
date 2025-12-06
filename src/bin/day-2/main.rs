use anyhow::{Result, anyhow};
use aoc_2025::core::*;
use itertools::Itertools;
use std::usize;

#[derive(Debug)]
struct Range {
    left: usize,
    right: usize,
}

impl TryFrom<&str> for Range {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (left, right) = value.split_once("-").ok_or("failed to parse range")?;

        Ok(Range {
            left: left
                .parse()
                .map_err(|_| format!("failed to parse left value in range {left}"))?,
            right: right
                .parse()
                .map_err(|_| format!("failed to parse right value in range {right}"))?,
        })
    }
}

impl Range {
    fn extract_invalid_reflections(&self) -> Vec<usize> {
        (self.left..=self.right)
            .into_iter()
            .filter(|x| Self::number_is_reflected(*x))
            .collect()
    }

    fn number_is_reflected(num: usize) -> bool {
        let digits = num.ilog10() + 1;
        if digits % 2 != 0 {
            return false;
        };
        let half_base = usize::pow(10, digits / 2);
        let left = num / half_base;
        let right = num - (left * half_base);

        left == right
    }

    fn extract_invalid_repititions(&self) -> Vec<usize> {
        (self.left..=self.right)
            .into_iter()
            .filter(|x| Self::number_contains_repeated_subsequence(*x))
            .collect()
    }

    fn number_contains_repeated_subsequence(num: usize) -> bool {
        // giddyup!
        'outer: for sub_sequence in 1..num.to_string().len() {
            let candidate = num
                .to_string()
                .chars()
                .into_iter()
                .take(sub_sequence)
                .join("");
            for next_sequence in num
                .to_string()
                .chars()
                .skip(sub_sequence)
                .map(String::from)
                .collect::<Vec<String>>()
                .chunks(sub_sequence)
            {
                if candidate != next_sequence.join("") {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }
}

fn parse_input_into_range(input: &str) -> Result<Vec<Range>> {
    input
        .split(",")
        .into_iter()
        .map(|x| Range::try_from(x).map_err(|x| anyhow!(x)))
        .collect()
}

fn problem_one(input: String) -> Result<()> {
    let range_collection = parse_input_into_range(&input)?;
    let result = range_collection
        .iter()
        .flat_map(|range| range.extract_invalid_reflections())
        .sum::<usize>();

    println!("{result}");
    Ok(())
}

fn problem_two(input: String) -> Result<()> {
    let range_collection = parse_input_into_range(&input)?;
    let result = range_collection
        .iter()
        .flat_map(|range| range.extract_invalid_repititions())
        .sum::<usize>();

    println!("{result}");
    Ok(())
}

fn main() -> Result<()> {
    let input = get_data("day-2")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_2_tests {
    use crate::parse_input_into_range;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_problem_one() -> anyhow::Result<()> {
        let range_collection = parse_input_into_range(INPUT)?;
        let result = range_collection
            .iter()
            .flat_map(|range| range.extract_invalid_reflections())
            .sum::<usize>();

        assert_eq!(result, 1227775554);
        Ok(())
    }

    #[test]
    fn test_problem_two() -> anyhow::Result<()> {
        let range_collection = parse_input_into_range(INPUT)?;
        let result = range_collection
            .iter()
            .flat_map(|range| range.extract_invalid_repititions())
            .sum::<usize>();
        assert_eq!(result, 4174379265);
        Ok(())
    }
}
