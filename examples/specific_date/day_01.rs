use std::collections::{HashMap, HashSet};

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
        let mut current_orientation = Orientation::North;
        let mut x = 0;
        let mut y = 0;

        for line in data.iter() {
            let orientation = self.get_orientation(
                current_orientation.clone(),
                line.chars().nth(0).unwrap().to_string().as_str(),
            );
            let movement: i32 = line[1..].to_string().parse().unwrap();

            match orientation {
                Orientation::North => y += movement,
                Orientation::South => y -= movement,
                Orientation::East => x += movement,
                Orientation::West => x -= movement,
            };

            current_orientation = orientation;
        }

        x.abs() + y.abs() // The real formula is |x2 - x1| + |y2 - y1| but we are omitting the (x1, y1) parts because of this point is on the coordinates (0, 0)
    }

    fn part_2(&self, data: Vec<String>) -> i32 {
        let mut current_orientation = Orientation::North;
        let mut x = 0;
        let mut y = 0;

        let mut positions: Vec<(i32, i32)> = Vec::new();
        let mut visited: HashMap<String, (i32, i32)> = HashMap::new();

        positions.push((0, 0));

        for line in data.iter() {
            let orientation = self.get_orientation(
                current_orientation.clone(),
                line.chars().nth(0).unwrap().to_string().as_str(),
            );
            let movement: i32 = line[1..].to_string().parse().unwrap();

            match orientation {
                Orientation::North => y += movement,
                Orientation::South => y -= movement,
                Orientation::East => x += movement,
                Orientation::West => x -= movement,
            };

            current_orientation = orientation;

            positions.push((x, y));
        }

        let mut crossing_position: Option<(i32, i32)> = None;
        for (index, current_position) in positions.iter().enumerate() {
            if index == positions.len() - 1 || crossing_position.is_some() {
                break;
            }

            let end_position = positions[index + 1];

            // Get all the positions between the current_position and the end_position
            for x in current_position.0 + 1..=end_position.0 {
                let key = format!("{},{}", x, current_position.1);
                visited.insert(key, (x, current_position.1));
            }

            for y in current_position.1 + 1..=end_position.1 {
                let key = format!("{},{}", current_position.0, y);
                if visited.contains_key(&key) {
                    crossing_position = Some((current_position.0, y));
                    break;
                }
                visited.insert(key, (current_position.0, y));
            }
        }

        let pos = crossing_position.unwrap_or_default();

        pos.0.abs() - pos.1.abs()
    }

    fn parse_input(&self, lines: Vec<String>) -> Vec<String> {
        let lines: Vec<String> = lines[0].split(",").map(|i| i.trim().to_owned()).collect();
        lines
    }
}
