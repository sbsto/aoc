use std::{collections::{BTreeMap, HashSet}, cmp::min};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Ord, PartialOrd)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl Card {
    fn winning_number_count(&self) -> u32 {
        count_intersection(&self.numbers, &self.winning_numbers)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);

    println!("Answer: {}", result);
}

fn part_2(input: &str) -> u32 {
    let cards = parse_text(input);
    let mut card_count_map: BTreeMap<Card, usize> =
        cards.iter().fold(BTreeMap::new(), |mut acc, card| {
            acc.insert(card.clone(), 1);
            acc
        });

    // iterate through each card, getting the winning number count, n
    // for the next n cards, increase the count by 1
    for card in &cards {
        let winning_number_count = card.winning_number_count();
        let existing_count = *card_count_map.get(card).expect("should exist");

        for i in card.id..min(card.id + winning_number_count, cards.len() as u32) {
            if let Some(count) = card_count_map.get_mut(&cards[i as usize]) {
                *count += existing_count;
            }
        }
    }

    card_count_map
        .iter()
        .fold(0, |acc, (_, count)| acc + *count as u32)
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
        let result = part_2(input);
        assert_eq!(result, 30);
    }
}
