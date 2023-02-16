use std::{error::Error, path::PathBuf};

use chrono::Datelike;
use clap::Parser;
use opts::RawOptions;

#[macro_use]
extern crate clap;

mod opts;

pub fn init() -> Result<(), std::io::Error> {
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

    // Run the exercise

    Ok(())
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
