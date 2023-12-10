use itertools::Itertools;

#[derive(Debug)]
struct Range {
    source_range: (isize, isize),
    target_range: (isize, isize),
}

impl Range {
    fn new(target_range: (isize, isize), source_range: (isize, isize)) -> Self {
        Self {
            source_range,
            target_range,
        }
    }

    fn contains_target(&self, value: isize) -> bool {
        value >= self.target_range.0 && value <= self.target_range.1
    }

    fn get_source_value(&self, value: isize) -> Option<isize> {
        if !self.contains_target(value) {
            return None;
        }

        let target_value_offset = value - self.target_range.0;
        let source_value = self.source_range.0 + target_value_offset;

        Some(source_value)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn get_source(&self, value: isize) -> isize {
        self.ranges
            .iter()
            .find_map(|range| range.get_source_value(value))
            .unwrap_or(value)
    }
}

type SourceRange = (isize, isize);

trait Container {
    fn contains(&self, value: isize) -> bool;
}

impl Container for SourceRange {
    fn contains(&self, value: isize) -> bool {
        value >= self.0 && value <= self.1
    }
}

#[derive(Debug)]
struct Data {
    source_numbers: Vec<(isize, isize)>,
    maps: Vec<Map>,
}

fn main() {
    let input = include_str!("input.txt");
    let result = process(input);

    println!("Answer: {:?}", result);
}

fn process(input: &str) -> isize {
    let data = parse_text(input);
    let reversed_maps = data.maps.iter().rev().collect_vec();

    let mut final_from_seed: Option<isize> = Option::None;
    let mut count = 0;

    while final_from_seed.is_none() {
        let mut location = count;
        for map in reversed_maps.iter() {
            location = map.get_source(location);
        }

        if data.source_numbers.iter().any(|range| range.contains(location)) {
            final_from_seed = Some(count);
        }

        count += 1;
    }

    final_from_seed.expect("should have found a seed")
}

fn parse_text(input: &str) -> Data {
    let lines = input
        .lines()
        .map(|line| line.to_string())
        .collect_vec();

    let source_ranges = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .filter_map(|number| number.parse::<isize>().ok())
        .collect_vec();

    let source_numbers  = source_ranges
        .iter()
        .enumerate()
        .filter_map(|(i, &number)| {
            if i % 2 != 0 {
                let range = (source_ranges[i - 1], source_ranges[i - 1] + number - 1);
                Some(range)
            } else {
                None
            }
        })
        .collect_vec();

    let mut maps: Vec<Map> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let mut ranges: Vec<Range> = Vec::new();

        if line.contains("map") {
            let range_lines: Vec<_> = lines
                .iter()
                .skip(i + 1)
                .take_while(|line| !line.is_empty())
                .collect();

            ranges = range_lines
                .iter() // This converts the iterator to a parallel iterator
                .map(|line| get_range(line))
                .collect();

            // index_offset is not needed anymore as we collect directly into ranges
        }

        if !ranges.is_empty() {
            maps.push(Map::new(ranges));
        }
    }

    Data {
        source_numbers,
        maps,
    }
}

fn get_range(input: &str) -> Range {
    let nums = input
        .split(' ')
        .filter_map(|number| number.parse::<isize>().ok())
        .collect::<Vec<isize>>();

    Range::new(
        (nums[0], nums[0] + nums[2] - 1),
        (nums[1], nums[1] + nums[2] - 1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("input_test.txt");
        let result = process(input);
        assert_eq!(result, 46);
    }

    #[test]
    fn test2() {
        let input = include_str!("input_test.txt");
        let data = parse_text(input);
        let reversed_maps = data.maps.iter().rev().collect::<Vec<&Map>>();
        let mut location = 86;
        for map in reversed_maps.iter() {
            location = map.get_source(location);
        }

        assert_eq!(location, 55);
    }
}
