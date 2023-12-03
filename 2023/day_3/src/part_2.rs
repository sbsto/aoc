use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    value: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Point>>,
}

#[derive(Debug, Clone)]
struct Number {
    value: usize,
    unique_neighbours: Vec<Point>,
}

impl Grid {
    fn new(data: Vec<Vec<Point>>) -> Self {
        Self { data }
    }

    fn width(&self) -> usize {
        self.data[0].len() - 1
    }

    fn height(&self) -> usize {
        self.data.len() - 1
    }

    fn neighbours(&self, x: usize, y: usize) -> Vec<&Point> {
        let mut result = Vec::new();
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for &(dx, dy) in &offsets {
            let x_with_offset = x as isize + dx;
            let y_with_offset = y as isize + dy;

            // check if neighbout is within the grid bounds
            if x_with_offset >= 0
                && x_with_offset < self.width() as isize
                && y_with_offset >= 0
                && y_with_offset < self.height() as isize
            {
                result.push(&self.data[y_with_offset as usize][x_with_offset as usize]);
            }
        }

        result
    }

    fn extract_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();

        // Iterate over each row in the grid
        for row in &self.data {
            let mut current_number = Vec::new();
            let mut current_value = 0;

            for point in row {
                if point.value.is_digit(10) {
                    current_number.push(point.clone());
                    current_value = current_value * 10
                        + point.value.to_digit(10).expect("should be a number") as usize;
                } else if !current_number.is_empty() {
                    numbers.push(Number {
                        value: current_value,
                        unique_neighbours: current_number
                            .iter()
                            .flat_map(|point| self.neighbours(point.x, point.y))
                            .filter(|point| point.value == '*')
                            .unique()
                            .cloned()
                            .collect::<Vec<Point>>(),
                    });

                    current_number.clear();
                    current_value = 0;
                }
            }

            if !current_number.is_empty() {
                numbers.push(Number {
                    value: current_value,
                    unique_neighbours: current_number
                        .iter()
                        .flat_map(|point| self.neighbours(point.x, point.y))
                        .filter(|point| point.value == '*')
                        .unique()
                        .cloned()
                        .collect::<Vec<Point>>(),
                });
            }
        }

        numbers
    }

    fn gear_map(&self) -> HashMap<Point, Vec<Number>> {
        let mut map: HashMap<Point, Vec<Number>> = HashMap::new();

        for number in self.extract_numbers() {
            for neighbour in &number.unique_neighbours {
                map.entry(neighbour.clone())
                    .and_modify(|numbers| numbers.push(number.clone()))
                    .or_insert(vec![number.clone()]);
            }
        }

        map
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);

    println!("Answer: {}", result);
}

fn part_2(input: &str) -> usize {
    let grid = parse_text(input);
    let point_map = grid.gear_map();

    let mut total = 0;
    point_map.iter().for_each(|(_, numbers)| {
        if numbers.len() == 2 {
            total += numbers[0].value * numbers[1].value;
        }
    });

    total
}

fn parse_text(input: &str) -> Grid {
    let data = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, value)| Point { value, x, y })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    Grid::new(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input_test.txt");
        let result = part_2(input);
        assert_eq!(result, 467835);
    }
}
