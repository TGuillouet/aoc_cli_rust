use aoc_cli::runner::RunnableDay;

#[derive(Debug, Default)]
pub struct Day01 {}

impl RunnableDay for Day01 {
    fn part_1(&self, data: Vec<String>) -> i32 {
        0
    }

    fn part_2(&self, data: Vec<String>) -> i32 {
        0
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<String> {
        let lines: Vec<String> = lines[0].split(",").map(|i| i.to_owned()).collect();
        lines
    }
}
