pub trait RunnableDay {
    fn part_1(&self, data: Vec<String>) -> i32;
    fn part_2(&self, data: Vec<String>) -> i32;

    fn parse_input(&self, lines: Vec<String>) -> Vec<String>;
}
