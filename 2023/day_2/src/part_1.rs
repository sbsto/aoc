#[derive(Debug)]
struct Game {
    id: String,
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

struct Limit {
    red: usize,
    blue: usize,
    green: usize,
}

const LIMIT: Limit = Limit {
    red: 12,
    blue: 14,
    green: 13,
};

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);

    println!("Sum: {}", result);
}

fn part_1(input: &str) -> usize {
    let games = parse_text(input);
    let id_sum = games
        .iter()
        .filter(|&game| {
            game.red.iter().all(|val| val <= &LIMIT.red)
                && game.blue.iter().all(|val| val <= &LIMIT.blue)
                && game.green.iter().all(|val| val <= &LIMIT.green)
        })
        .map(|game| {
            game.id
                .clone()
                .parse::<usize>()
                .expect("id should be a number")
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum::<usize>();

    id_sum
}

fn parse_text(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    input.lines().for_each(|x| {
        let mut red: Vec<usize> = Vec::new();
        let mut blue: Vec<usize> = Vec::new();
        let mut green: Vec<usize> = Vec::new();

        let line_parsed = x.parse::<String>().expect("should be a string");
        let split_at_game = line_parsed.split(":").collect::<Vec<&str>>();
        let id = split_at_game
            .first()
            .expect("should have game")
            .chars()
            .filter(|ch| ch.is_digit(10))
            .collect::<String>();

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
            id: id.to_string(),
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
        let result = part_1(input);
        assert_eq!(result, 8);
    }
}
