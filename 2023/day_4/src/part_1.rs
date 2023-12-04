use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    fn intersection_count(&self) -> u32 {
        count_intersection(&self.numbers, &self.winning_numbers)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);

    println!("Answer: {}", result);
}

fn part_1(input: &str) -> u32 {
    let cards = parse_text(input);

    cards.iter().fold(0, |acc, card| {
        let intersection_count = card.intersection_count();
        if intersection_count == 0 {
            acc
        } else {
            acc + 2u32.pow(intersection_count - 1)
        }
    })
}

fn parse_text(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let parts = line.split(": ").collect::<Vec<&str>>();
            let id = parts
                .first()
                .expect("id section should exist")
                .split(" ")
                .last()
                .expect("id should exist")
                .parse::<u32>()
                .expect("should be a number");

            let numbers_section = parts
                .last()
                .expect("numbers section should exist")
                .split("|")
                .collect::<Vec<&str>>();

            let numbers = numbers_section
                .last()
                .expect("numbers should exist")
                .split(" ")
                .filter_map(|n| {
                    if n.is_empty() {
                        return None;
                    }

                    Some(n.parse::<u32>().expect("should be a number"))
                })
                .collect::<Vec<u32>>();

            let winning_numbers = numbers_section
                .first()
                .expect("winning numbers should exist")
                .split(" ")
                .filter_map(|n| {
                    if n.is_empty() {
                        return None;
                    }

                    Some(n.parse::<u32>().expect("should be a number"))
                })
                .collect::<Vec<u32>>();

            Card {
                id,
                numbers,
                winning_numbers,
            }
        })
        .collect::<Vec<Card>>()
}

fn count_intersection<T: Eq + std::hash::Hash>(list1: &[T], list2: &[T]) -> u32 {
    let set1: HashSet<_> = list1.iter().collect();
    let set2: HashSet<_> = list2.iter().collect();

    set1.intersection(&set2).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input_test.txt");
        let result = part_1(input);
        assert_eq!(result, 13);
    }
}
