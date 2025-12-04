use std::{env, fs};
use anyhow::{Result};
use clap::{Parser, ValueEnum};

pub fn get_data(day: &str) -> Result<String> {
    let path = format!(
        "{}/src/bin/{}/input.txt",
        env::current_dir()?.display(),
        day
    );

    Ok(fs::read_to_string(path)?)
}

pub fn get_lines(day: &str) -> Result<Vec<String>> {
    Ok(get_data(day)?.lines().map(String::from).collect())
}

#[derive(Debug, PartialEq, ValueEnum, Clone)]
enum Problem {
    One,
    Two
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = Problem::One)]
    problem: Problem
}

pub fn run_problems(
    problem_one: fn(Vec<String>) -> Result<()>,
    problem_two: fn(Vec<String>) -> Result<()>,
    input: Vec<String>
) -> Result<()> {
    let args = Args::parse();
    
    match args.problem { 
        Problem::One => problem_one(input),
        Problem::Two => problem_two(input)
    }
}