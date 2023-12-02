#[derive(Debug)]
struct Game {
    red: Vec<usize>,
    blue: Vec<usize>,
    green: Vec<usize>,
}

#[derive(Debug)]
struct Turn {
    red: usize,
    blue: usize,
    green: usize,
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);

    println!("Sum: {}", result);
}

fn part_2(input: &str) -> usize {
    parse_text(input)
        .iter()
        .map(|game| {
            let max_red = game.red.iter().max().expect("should have max");
            let max_blue = game.blue.iter().max().expect("should have max");
            let max_green = game.green.iter().max().expect("should have max");

            max_red * max_blue * max_green
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>()
}

fn parse_text(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    input.lines().for_each(|x| {
        let mut red: Vec<usize> = Vec::new();
        let mut blue: Vec<usize> = Vec::new();
        let mut green: Vec<usize> = Vec::new();

        let line_parsed = x.parse::<String>().expect("should be a string");
        let split_at_game = line_parsed.split(":").collect::<Vec<&str>>();

        split_at_game
            .last()
            .expect("should have turns")
            .split(";")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|turn| {
                let mut turn_totals = Turn {
                    red: 0,
                    blue: 0,
                    green: 0,
                };
                turn.split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .for_each(|count_string| {
                        let count = count_string
                            .chars()
                            .filter(|&c| c.is_digit(10))
                            .collect::<String>()
                            .parse::<usize>()
                            .expect("should be a number");

                        if count_string.contains("red") {
                            turn_totals.red += count;
                        } else if count_string.contains("blue") {
                            turn_totals.blue += count;
                        } else if count_string.contains("green") {
                            turn_totals.green += count;
                        }
                    });

                red.push(turn_totals.red);
                blue.push(turn_totals.blue);
                green.push(turn_totals.green);
            });

        let game = Game {
            red,
            blue,
            green,
        };

        games.push(game);
    });

    games
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input_test.txt");
        let result = part_2(input);
        assert_eq!(result, 2286);
    }
}
