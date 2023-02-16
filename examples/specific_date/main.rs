use std::collections::HashMap;

use aoc_cli::runner::RunnableDay;
use day_01::Day01;

mod day_01;

fn main() {
    let mut days: HashMap<u32, Box<dyn RunnableDay>> = HashMap::new();
    days.insert(1, Box::new(Day01::default()));

    let aoc_result = aoc_cli::init(days);

    match aoc_result {
        Ok(_) => println!("Execution finished"),
        Err(error) => eprintln!("Error while executing this day: {}", error),
    }
}
