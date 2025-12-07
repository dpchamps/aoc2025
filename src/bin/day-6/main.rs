use anyhow::{Result, anyhow};
use aoc_2025::core::*;
use itertools::Itertools;
use std::fmt::{Display, Formatter, write};
use std::usize;
use regex::Regex;

#[derive(Debug)]
struct Matrix(Vec<Vec<String>>);

impl From<&str> for Matrix {
    fn from(value: &str) -> Self {
        let re = Regex::new(r" {2,}\S+|\S+").unwrap();
        let inner = value
            .lines()
            .map(|line| re.find_iter(line).map(|x| x.as_str().to_string()).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();

        Matrix(inner)
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<String>>);

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let re = Regex::new(r" {2,}\S+|\S+").unwrap();
        let inner = value
            .lines()
            .map(|line| line.chars().map(String::from).collect::<Vec<String>>())
            .collect::<Vec<Vec<String>>>();

        Grid(inner)
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let m = self.0.iter().map(|row| {
            let r = row.iter().join(" ");
            String::from(r.trim())
        }).join("\n");

        write!(f, "--\n{}\n--", m)
    }
}

impl Matrix {
    pub fn new(inner: Vec<Vec<String>>) -> Self {
        Self(inner)
    }
    
    fn transpose(&mut self){
        let n_rows = self.0.len();
        let n_cols = self.0[0].len();

        let mut transposed = vec![vec![String::from(" "); n_rows]; n_cols];
        for y in 0..n_rows {
            for x in 0..n_cols {
                transposed[x][y] = self.0[y][x].clone();
            }
        }

        self.0 = transposed;
    }

    pub fn rotate_clockwise(&mut self) {
        self.transpose();

        for row in self.0.iter_mut() {
            row.reverse();
        }
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.transpose();

        self.0.reverse();
    }
}

#[derive(Debug)]
struct MathBook {
    operations: Vec<Operation>,
}

impl TryFrom<Matrix> for MathBook {
    type Error = ();

    fn try_from(value: Matrix) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            operations: value
                .0
                .iter()
                .map(Operation::try_from)
                .collect::<Result<Vec<Operation>, ()>>()?,
        })
    }
}

impl MathBook {
    pub fn total(&self) -> usize {
        self.operations.iter().map(Operation::apply).sum()
    }

    pub fn cephalopod_columns(&mut self) {
        for operation in self.operations.iter_mut() {
            operation.cephalopod();
        }
    }
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    elements: Vec<String>,
}

impl TryFrom<&Vec<String>> for Operation {
    type Error = ();

    fn try_from(value: &Vec<String>) -> std::result::Result<Self, Self::Error> {
        let (x, ys) = value.split_first().ok_or(())?;
        println!("{x}, {:?}", ys);

        Ok(Self {
            operator: Operator::try_from(x)?,
            elements: ys
                .iter()
                .map(String::from)
                .collect(),
        })
    }
}

impl Operation {
    pub fn apply(&self) -> usize {
        let numbers = self.elements.iter().map(|x| x.trim().parse::<usize>().expect(&format!("failed to parse number {x}")));
        match self.operator {
            Operator::MULT => numbers.fold(1, |acc, el| acc * el),
            Operator::PLUS => numbers.sum(),
        }
    }

    pub fn cephalopod(&mut self) {
        self.elements.reverse();
        // let max_len = self
        //     .elements
        //     .iter()
        //     .map(|x| x.to_string().len())
        //     .max()
        //     .expect("");
        let exploded_elements = self
            .elements
            .iter()
            .map(|x| {
                x
                    .chars()
                    // .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        let mut matrix = Matrix::new(exploded_elements);
        println!("{:?}", matrix);

        matrix.rotate_counter_clockwise();
        println!("{}", matrix);
    }
}

#[derive(Debug)]
enum Operator {
    PLUS,
    MULT,
}

impl TryFrom<&String> for Operator {
    type Error = ();

    fn try_from(value: &String) -> std::result::Result<Self, Self::Error> {
        match value.as_str().trim() {
            "*" => Ok(Operator::MULT),
            "+" => Ok(Operator::PLUS),
            _ => Err(()),
        }
    }
}

fn problem_one(input: String) -> Result<()> {
    let mut matrix = Matrix::from(input.as_str());
    matrix.rotate_clockwise();
    println!("{matrix}");
    let mathbook = MathBook::try_from(matrix).map_err(|_| anyhow!(""))?;

    println!("{}", mathbook.total());
    Ok(())
}

fn problem_two(input: String) -> Result<()> {
    let mut matrix = Matrix::from(input.as_str());
    matrix.rotate_clockwise();
    println!("{matrix}");
    let mut mathbook = MathBook::try_from(matrix).map_err(|_| anyhow!(""))?;
    mathbook.cephalopod_columns();
    println!("{:?}", mathbook);
    Ok(())
}

fn main() -> Result<()> {
    let input = get_data("day-6")?;
    run_problems(problem_one, problem_two, input)?;
    Ok(())
}

#[cfg(test)]
mod day_6_tests {
    use crate::{Grid, MathBook, Matrix};
    use anyhow::anyhow;

    const INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_problem_one() -> anyhow::Result<()> {
        let mut matrix = Matrix::from(INPUT);
        matrix.rotate_clockwise();
        println!("m: {}", matrix);
        let mathbook = MathBook::try_from(matrix).map_err(|_| anyhow!("falied to parse mathbook"))?;
        println!("{:?}", mathbook);

        assert_eq!(mathbook.total(), 4277556);

        Ok(())
    }

    #[test]
    fn test_problem_two() -> anyhow::Result<()> {
        let mut matrix = Grid::from(INPUT);
        println!("{:?}", matrix);
        // let mut mathbook = MathBook::try_from(matrix).map_err(|_| anyhow!(""))?;
        // mathbook.cephalopod_columns();
        // println!("{:?}", mathbook);

        panic!();
        // assert_eq!(mathbook.total(), 4277556);
        Ok(())
    }
}
