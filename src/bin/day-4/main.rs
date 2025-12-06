use anyhow::Result;
use aoc_2025::core::*;
use std::collections::{HashMap, HashSet};

type MapType = HashMap<(usize, usize), Option<()>>;

#[derive(Debug)]
struct Map {
    elements: MapType,
    paper_positions: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut hash_map: MapType = HashMap::new();
        let mut paper_positions: HashSet<(usize, usize)> = HashSet::new();
        for (y, line) in value.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let element = if char == '@' {
                    paper_positions.insert((x, y));
                    Some(())
                } else {
                    None
                };
                hash_map.insert((x, y), element);
            }
        }

        Map {
            elements: hash_map,
            paper_positions,
            width: value.lines().next().unwrap().len(),
            height: value.lines().count(),
        }
    }
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

impl Map {
    pub fn count_neighbors(&self, x: i32, y: i32) -> usize {
        DIRECTIONS
            .iter()
            .map(|&(dx, dy)| ((x + dx), (y + dy)))
            .filter(|&(nx, ny)| {
                if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                    return false;
                }

                let element = self
                    .elements
                    .get(&(nx as usize, ny as usize))
                    .expect("something terrible happened");

                element.is_some()
            })
            .count()
    }

    pub fn is_roll_accessible(&self, (pos_x, pos_y): &(usize, usize)) -> bool {
        self.count_neighbors(*pos_x as i32, *pos_y as i32) < 4
    }

    pub fn count_accessible_rolls(&self) -> usize {
        self.paper_positions
            .iter()
            .filter(|pos| self.is_roll_accessible(*pos))
            .count()
    }

    pub fn exaust_all_accessible_rolls(&mut self) -> usize {
        let candidates = self
            .paper_positions
            .clone()
            .into_iter()
            .filter(|pos| self.is_roll_accessible(pos))
            .collect::<HashSet<(usize, usize)>>();
        let candidates_len = candidates.iter().count();
        if candidates_len == 0 {
            return 0;
        }
        let next_paper_positons = self
            .paper_positions
            .iter()
            .filter(|pos| !candidates.contains(pos))
            .map(|&(x, y)| (x, y))
            .collect::<HashSet<(usize, usize)>>();
        for (candidate_x, candidate_y) in candidates {
            self.elements.insert((candidate_x, candidate_y), None);
        }

        self.paper_positions = next_paper_positons;

        candidates_len + self.exaust_all_accessible_rolls()
    }
}

fn problem_one(input: String) -> Result<()> {
    let map = Map::from(input.as_str());
    println!("{}", map.count_accessible_rolls());
    Ok(())
}

fn problem_two(input: String) -> Result<()> {
    let mut map = Map::from(input.as_str());
    println!("{}", map.exaust_all_accessible_rolls());
    Ok(())
}

fn main() -> Result<()> {
    let input = get_data("day-4")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_4_tests {
    use crate::Map;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_problem_one() -> anyhow::Result<()> {
        let map = Map::from(INPUT);
        let result = map.count_accessible_rolls();

        assert_eq!(result, 13);
        Ok(())
    }

    #[test]
    fn test_problem_two() -> anyhow::Result<()> {
        let mut map = Map::from(INPUT);
        let result = map.exaust_all_accessible_rolls();

        assert_eq!(result, 43);
        Ok(())
    }
}
