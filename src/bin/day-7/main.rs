use anyhow::Result;
use aoc_2025::core::*;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::usize;

#[derive(Debug)]
enum Tile {
    Manifold,
    Splitter,
    Beam,
    Empty,
}

impl Tile {
    pub fn render(&self) -> String {
        match self {
            Tile::Manifold => "S".to_string(),
            Tile::Splitter => "^".to_string(),
            Tile::Beam => "|".to_string(),
            Tile::Empty => ".".to_string(),
        }
    }
}

#[derive(Debug)]
struct Grid {
    active_splitters: HashSet<(usize, usize)>,
    elements: HashMap<(usize, usize), Tile>,
    manifold: (usize, usize),
    active_beams: HashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut elements: HashMap<(usize, usize), Tile> = HashMap::new();
        let mut manifold: Option<(usize, usize)> = None;

        for (y, row) in value.lines().enumerate() {
            for (x, el) in row.chars().enumerate() {
                let tile = match el {
                    'S' => {
                        manifold = Some((x, y));
                        Tile::Manifold
                    }
                    '^' => Tile::Splitter,
                    _ => Tile::Empty,
                };

                elements.insert((x, y), tile);
            }
        }

        Self {
            active_splitters: HashSet::new(),
            elements,
            manifold: manifold.expect("expected to find a manifold but didn't find one!"),
            active_beams: HashSet::new(),
            width: value.lines().next().unwrap().chars().count(),
            height: value.lines().count(),
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec![String::from("."); self.width]; self.height];
        for ((x, y), tile) in &self.elements {
            grid[*y][*x] = tile.render()
        }

        let output = grid.iter().map(|row| row.join("")).join("\n");

        write!(f, "{}", output)
    }
}

impl Grid {
    pub fn activate(&mut self) {
        self.propagate_beam(self.manifold);

        while !self.active_beams.is_empty() {
            let next_active_beams = self
                .active_beams
                .clone()
                .into_iter()
                .collect::<Vec<(usize, usize)>>();
            next_active_beams
                .iter()
                .for_each(|beam| self.propagate_beam(*beam));
        }
    }

    pub fn propagate_beam(&mut self, coord: (usize, usize)) {
        if let Some(next_beam_pos) = self.calculate_next_pos(coord, (0, 1)) {
            let tile = self
                .elements
                .get(&next_beam_pos)
                .expect("expected tile, but no tile!");

            match tile {
                &Tile::Empty => {
                    self.set_tile(next_beam_pos, Tile::Beam);
                }
                &Tile::Splitter => {
                    self.active_splitters.insert(next_beam_pos);
                    self.maybe_set_tile_at(next_beam_pos, (-1, 0), Tile::Beam);
                    self.maybe_set_tile_at(next_beam_pos, (1, 0), Tile::Beam);
                }
                _ => {}
            };
        }

        self.active_beams.remove(&coord);
    }

    pub fn maybe_set_tile_at(
        &mut self,
        coord: (usize, usize),
        direction: (isize, isize),
        tile: Tile,
    ) {
        if let Some(next) = self.calculate_next_pos(coord, direction) {
            self.set_tile(next, tile);
        }
    }

    pub fn set_tile(&mut self, (c_x, c_y): (usize, usize), tile: Tile) {
        match tile {
            Tile::Beam => {
                self.active_beams.insert((c_x, c_y));
            }
            _ => {}
        }
        self.elements.insert((c_x, c_y), tile);
    }

    pub fn calculate_next_pos(
        &self,
        (c_x, c_y): (usize, usize),
        (d_x, d_y): (isize, isize),
    ) -> Option<(usize, usize)> {
        let next_x = c_x as isize + d_x;
        let next_y = c_y as isize + d_y;
        if next_x < 0
            || next_y < 0
            || next_x >= self.width as isize
            || next_y >= self.height as isize
        {
            return None;
        };

        Some((next_x as usize, next_y as usize))
    }

    pub fn many_worlds(&self) -> usize {
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

        self.many_world_rec(self.manifold, &mut visited)
    }

    pub fn many_world_rec(
        &self,
        current: (usize, usize),
        visited: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(cached_child_paths) = visited.get(&current) {
            return *cached_child_paths;
        }

        let tile = self
            .elements
            .get(&current)
            .expect("something terrible happened");

        let next_positions = match tile {
            Tile::Splitter => {
                let left = self.calculate_next_pos(current, (-1, 0));
                let right = self.calculate_next_pos(current, (1, 0));

                vec![left, right]
            }
            _ => {
                vec![self.calculate_next_pos(current, (0, 1))]
            }
        };

        let mut path_count = 0;

        for maybe_next in next_positions {
            if let Some(next_coord) = maybe_next {
                path_count += self.many_world_rec(next_coord, visited)
            } else {
                path_count += 1;
            }
        }

        visited.insert(current, path_count);

        path_count
    }
}

fn problem_one(input: String) -> Result<()> {
    let mut grid = Grid::from(input.as_str());
    grid.activate();
    println!("{}", grid);
    println!("{:#?}", grid.active_splitters.len());
    Ok(())
}

fn problem_two(input: String) -> Result<()> {
    let mut grid = Grid::from(input.as_str());
    grid.activate();
    println!("{}", grid);
    println!("{:#?}", grid.many_worlds());
    Ok(())
}

fn main() -> Result<()> {
    let input = get_data("day-7")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_7_tests {
    use crate::Grid;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_problem_one() -> anyhow::Result<()> {
        let mut grid = Grid::from(INPUT);
        grid.activate();
        assert_eq!(grid.active_splitters.len(), 21);

        Ok(())
    }

    #[test]
    fn test_problem_two() -> anyhow::Result<()> {
        let mut grid = Grid::from(INPUT);
        grid.activate();
        assert_eq!(grid.many_worlds(), 40);

        Ok(())
    }
}
