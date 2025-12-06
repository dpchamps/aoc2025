use anyhow::{Result, anyhow};
use aoc_2025::core::*;
use std::cmp::min;

struct Bank(Vec<u32>);

impl TryFrom<&str> for Bank {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Bank(
            value
                .chars()
                .map(|x| {
                    x.to_digit(10)
                        .ok_or(format!("failed to parse battery bank {value}"))
                })
                .collect::<Result<Vec<u32>, String>>()?,
        ))
    }
}

impl Bank {
    pub fn turn_on(&self, k: usize) -> usize {
        let result = Bank::turn_on_inner(&self.0, k);
        result
    }

    pub fn turn_on_inner(bank: &[u32], k: usize) -> usize {
        if k == 0 {
            return 0;
        }

        let possible_base = usize::pow(10, k as u32 - 1);

        let (next, next_idx) = bank
            .iter()
            .enumerate()
            .map(|(idx, n)| {
                let exp = bank.len() - idx - 1;
                let max_exp = usize::min(exp, k + 1);
                let real_base = usize::pow(10, max_exp as u32);
                (
                    idx,
                    min((*n as usize) * real_base, (*n as usize) * possible_base),
                )
            })
            .fold(None, |acc, (idx, quantity)| match acc {
                None => Some((quantity, idx)),
                Some((last_highest_quantity, _)) => {
                    if last_highest_quantity >= quantity {
                        acc
                    } else {
                        Some((quantity, idx))
                    }
                }
            })
            .expect("impossible");

        let next_bank = bank
            .iter()
            .skip(next_idx + 1)
            .map(|x| *x)
            .collect::<Vec<u32>>();

        next + Bank::turn_on_inner(&next_bank, k - 1)
    }
}

fn parse_into_banks(input: String) -> Result<Vec<Bank>> {
    input
        .lines()
        .map(|x| Bank::try_from(x).map_err(|y| anyhow!(y)))
        .collect::<Result<Vec<Bank>>>()
}

fn problem_one(input: String) -> Result<()> {
    let banks = parse_into_banks(input)?;
    let result = banks.iter().map(|bank| bank.turn_on(2)).sum::<usize>();

    println!("{result}");

    Ok(())
}

fn problem_two(input: String) -> Result<()> {
    let banks = parse_into_banks(input)?;
    let result = banks.iter().map(|bank| bank.turn_on(12)).sum::<usize>();

    println!("{result}");
    Ok(())
}

fn main() -> Result<()> {
    let input = get_data("day-3")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_3_tests {
    use crate::parse_into_banks;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_problem_one() -> anyhow::Result<()> {
        let banks = parse_into_banks(String::from(INPUT))?;
        let result = banks.iter().map(|bank| bank.turn_on(2)).sum::<usize>();
        assert_eq!(result, 357);
        Ok(())
    }

    #[test]
    fn test_problem_two() -> anyhow::Result<()> {
        let banks = parse_into_banks(String::from(INPUT))?;
        let result = banks.iter().map(|bank| bank.turn_on(12)).sum::<usize>();
        assert_eq!(result, 3121910778619);
        Ok(())
    }
}
