use std::collections::HashSet;

use aoc_cli::runner::RunnableDay;

#[derive(Debug, PartialEq, Clone)]
enum Orientation {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Default)]
pub struct Day01 {}

impl Day01 {
    fn traverse(&self, should_stop: bool, data: Vec<String>) -> (i32, i32) {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut current_orientation = Orientation::North;
        let mut x = 0;
        let mut y = 0;

        for line in data.iter() {
            let orientation = self.get_orientation(
                current_orientation.clone(),
                line.chars().nth(0).unwrap().to_string().as_str(),
            );
            let movement: i32 = line[1..].to_string().parse().unwrap();

            for _ in 0..movement {
                match orientation {
                    Orientation::North => y += 1,
                    Orientation::South => y -= 1,
                    Orientation::East => x += 1,
                    Orientation::West => x -= 1,
                };
                if should_stop && visited.contains(&(x, y)) {
                    return (x, y);
                }

                visited.insert((x, y));
            }
            current_orientation = orientation;
        }

        (x, y)
    }

    fn get_orientation(&self, current: Orientation, orientation_modifier: &str) -> Orientation {
        match orientation_modifier {
            "L" => {
                if current == Orientation::North {
                    Orientation::West
                } else if current == Orientation::West {
                    Orientation::South
                } else if current == Orientation::South {
                    Orientation::East
                } else {
                    Orientation::North
                }
            }
            "R" => {
                if current == Orientation::North {
                    Orientation::East
                } else if current == Orientation::East {
                    Orientation::South
                } else if current == Orientation::South {
                    Orientation::West
                } else {
                    Orientation::North
                }
            }
            _ => Orientation::North,
        }
    }
}

impl RunnableDay for Day01 {
    fn part_1(&self, data: Vec<String>) -> i32 {
        let (x, y) = self.traverse(false, data);
        x.abs() + y.abs() // The real formula is |x2 - x1| + |y2 - y1| but we are omitting the (x1, y1) parts because of this point is on the coordinates (0, 0)
    }

    fn part_2(&self, data: Vec<String>) -> i32 {
        let (x, y) = self.traverse(true, data);
        x.abs() + y.abs()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<String> {
        let lines: Vec<String> = lines[0].split(",").map(|i| i.trim().to_owned()).collect();
        lines
    }
}
