#[derive(Debug, Clone)]
struct Point {
    value: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<Point>>,
}

#[derive(Debug)]
struct Number {
    points: Vec<Point>,
    value: usize,
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
                        points: current_number.clone(),
                        value: current_value,
                    });

                    current_number.clear();
                    current_value = 0;
                }
            }

            if !current_number.is_empty() {
                numbers.push(Number {
                    points: current_number,
                    value: current_value,
                });
            }
        }

        numbers
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);

    println!("Answer: {}", result);
}

fn part_1(input: &str) -> usize {
    let grid = parse_text(input);
    let numbers = grid.extract_numbers();

    let mut total = 0;

    numbers.iter().for_each(|number| {
        let is_valid_number = number.points.iter().any(|point| {
            let neighbours = grid.neighbours(point.x, point.y);
            neighbours
                .iter()
                .any(|neighbour| neighbour.value != '.' && !neighbour.value.is_digit(10))
        });

        if is_valid_number {
            total += number.value;
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
        let result = part_1(input);
        assert_eq!(result, 4361);
    }
}
