use anyhow::{Result, anyhow};
use aoc_2025::core::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::usize;

#[derive(Debug)]
struct FoodDb {
    ranges: Vec<Range>,
    foods: Vec<usize>,
}

impl TryFrom<&str> for FoodDb {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (raw_ranges, raw_foods) = value.split_once("\n\n").ok_or("Failed to split food db")?;
        let mut ranges = raw_ranges
            .lines()
            .map(Range::try_from)
            .collect::<Result<Vec<Range>, String>>()?;
        let foods = raw_foods
            .lines()
            .map(|x| {
                x.parse::<usize>()
                    .map_err(|x| String::from("failed to parse a food into a number"))
            })
            .collect::<Result<Vec<usize>, String>>()?;

        ranges.sort();

        Ok(Self { ranges, foods })
    }
}

impl FoodDb {
    pub fn count_fresh_foods(&self) -> usize {
        let fresh_foods = self
            .foods
            .iter()
            .filter(|&food| self.ranges.iter().any(|range| range.includes(*food)))
            .collect::<Vec<&usize>>();

        fresh_foods.iter().count()
    }

    pub fn compact_ranges(&mut self) {
        let mut ranges = self.ranges.clone().into_iter();
        let mut next_ranges: Vec<Range> = Vec::new();
        let mut current_range: Option<Range> = None;

        loop {
            match ranges.next() {
                Some(next_range) => {
                    if let Some(current) = current_range {
                        if let Some(combined) = Range::combine(&current, &next_range) {
                            current_range = Some(combined)
                        } else {
                            next_ranges.push(current);
                            current_range = Some(next_range);
                        }
                    } else {
                        current_range = Some(next_range);
                    }
                }
                None => break,
            }
        }

        if let Some(last_range) = current_range {
            next_ranges.push(last_range)
        }

        self.ranges = next_ranges;
    }

    pub fn count_possible_fresh_ingredients(&self) -> usize {
        self.ranges.iter().map(Range::size).sum()
    }
}

#[derive(Debug, Eq, Clone)]
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

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.left.cmp(&other.left)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
    }
}

impl Range {
    pub fn includes(&self, el: usize) -> bool {
        self.left <= el && self.right >= el
    }

    pub fn size(&self) -> usize {
        (self.right - self.left) + 1
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    pub fn overlaps(a: &Range, b: &Range) -> bool {
        let min_right = usize::min(a.right, b.right);
        let max_left = usize::max(a.left, b.left);

        a.contains(b) || b.contains(a) || max_left <= min_right
    }

    pub fn combine(a: &Range, b: &Range) -> Option<Range> {
        let min_left = usize::min(a.left, b.left);
        let max_right = usize::max(a.right, b.right);

        if Range::overlaps(a, b) {
            return Some(Range {
                left: min_left,
                right: max_right,
            });
        }

        None
    }
}

fn problem_one(input: String) -> Result<()> {
    let db = FoodDb::try_from(input.as_str()).map_err(|x| anyhow!(x))?;
    let result = db.count_fresh_foods();
    println!("{result}");
    Ok(())
}

fn problem_two(input: String) -> Result<()> {
    let mut db = FoodDb::try_from(input.as_str()).map_err(|x| anyhow!(x))?;
    db.compact_ranges();
    let result = db.count_possible_fresh_ingredients();
    println!("{result}");
    Ok(())
}

fn main() -> Result<()> {
    let input = get_data("day-5")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_5_tests {
    use crate::FoodDb;
    use anyhow::anyhow;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_problem_one() -> anyhow::Result<()> {
        let db = FoodDb::try_from(INPUT).map_err(|x| anyhow!(x))?;
        let result = db.count_fresh_foods();

        assert_eq!(result, 3);
        Ok(())
    }

    #[test]
    fn test_problem_two() -> anyhow::Result<()> {
        let mut db = FoodDb::try_from(INPUT).map_err(|x| anyhow!(x))?;
        db.compact_ranges();
        let result = db.count_possible_fresh_ingredients();

        assert_eq!(result, 14);

        Ok(())
    }
}
