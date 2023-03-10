use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::BufRead,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::Datelike;
use clap::Parser;
use errors::DayNotFoundError;
use opts::RawOptions;
use runner::RunnableDay;

#[macro_use]
extern crate clap;

pub mod errors;
mod opts;
pub mod runner;

pub fn init(days: HashMap<u32, Box<dyn RunnableDay>>) -> Result<(), Box<dyn Error>> {
    let options = RawOptions::parse();

    // Create the exercise date from the specified inputs
    let current_date = chrono::Utc::now();
    let year = match options.year {
        Some(year) => year,
        None => current_date.year(),
    };
    let day = match options.day {
        Some(day) => day,
        None => current_date.day(),
    };

    let mut inputs_dir = get_inputs_dir(options.input_dir)?;
    inputs_dir.push(format!("{}", year));
    inputs_dir.push(format!("day_{:0>2}.txt", day));

    let day_executable = days.get(&day);

    match day_executable {
        Some(day_executable) => {
            let lines = read_lines(inputs_dir)?;
            let data = day_executable.parse_input(lines);

            let start = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let part1 = day_executable.part_1(data.clone());
            let end = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let time = end - start;
            println!(
                "Part 1: {}, executed in {} microseconds",
                part1,
                time.as_micros() as f64 / 1000.0
            );

            let start = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let part2 = day_executable.part_2(data);
            let end = SystemTime::now().duration_since(UNIX_EPOCH)?;
            let time = end - start;
            println!(
                "Part 2: {}, executed in {} microseconds",
                part2,
                time.as_micros() as f64 / 1000.0
            );

            Ok(())
        }
        None => {
            println!("Could not find this day in the runnables");
            Err(Box::new(DayNotFoundError))
        }
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(filename)?;
    let file_lines = std::io::BufReader::new(file).lines();

    for line in file_lines {
        if let Ok(line) = line {
            lines.push(line);
        }
    }

    Ok(lines)
}

fn get_inputs_dir(input_dir: Option<String>) -> Result<PathBuf, std::io::Error> {
    if let Some(input_dir) = input_dir {
        let mut path = PathBuf::new();
        path.push(input_dir);
        path.push("inputs");
        return Ok(path);
    }

    let mut current_dir = std::env::current_dir()?;
    current_dir.push("inputs");
    Ok(current_dir)
}
