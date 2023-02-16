use std::{collections::HashMap, error::Error, path::PathBuf};

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
            let data = day_executable.parse_input(vec![]);
            println!("{}", day_executable.part_1(data.clone()));
            println!("{}", day_executable.part_2(data));

            Ok(())
        }
        None => {
            println!("Could not find this day in the runnables");
            Err(Box::new(DayNotFoundError))
        }
    }
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
