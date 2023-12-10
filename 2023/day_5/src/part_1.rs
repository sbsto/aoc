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

    fn contains(&self, value: isize) -> bool {
        value >= self.source_range.0 && value <= self.source_range.1
    }

    fn get_target_value(&self, value: isize) -> Option<isize> {
        if !self.contains(value) {
            return None;
        }

        let source_value_offset = value - self.source_range.0;
        let target_value = self.target_range.0 + source_value_offset;

        Some(target_value)
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

    fn get(&self, value: isize) -> isize {
        for range in &self.ranges {
            if let Some(target_value) = range.get_target_value(value) {
                return target_value;
            }
        }

        value
    }
}

#[derive(Debug)]
struct Data {
    source_numbers: Vec<isize>,
    maps: Vec<Map>,
}

fn main() {
    let input = include_str!("input.txt");
    let result = process(input);

    println!("Answer: {:?}", result);
}

fn process(input: &str) -> isize {
    let data = parse_text(input);

    let mut final_numbers: Vec<isize> = Vec::new();
    for number in data.source_numbers {
        let mut target_number = number;
        for map in &data.maps {
            target_number = map.get(target_number);
        }

        final_numbers.push(target_number);
    }

    *final_numbers.iter().min().expect("Should have a minimum")
}

fn parse_text(input: &str) -> Data {
    let lines = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();

    let source_numbers = lines[0].split(":").collect::<Vec<&str>>()[1]
        .split(" ")
        .filter_map(|number| number.parse::<isize>().ok())
        .collect::<Vec<isize>>();

    let mut maps: Vec<Map> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let mut ranges: Vec<Range> = Vec::new();
        let mut index_offset = 1;
        if line.contains("map") {
            while i + index_offset < lines.len() && !lines[i + index_offset].is_empty() {
                let range = get_range(&lines[i + index_offset]);
                ranges.push(range);
                index_offset += 1;
            }
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
        .split(" ")
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
        assert_eq!(result, 35);
    }
}
