fn main() {
    let input = include_str!("input_1.txt");
    let result = part_2(input);
    println!("Result: {}", result);
}


fn part_2(input: &str) -> u32 {
    input.lines().map(|x| x.to_string()).map(process_line_b).sum::<u32>()
}

fn process_line_b(line: String) -> u32 {
    let new_line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    let mut filtered_chars = new_line.chars().filter(|c| c.is_digit(10));

    let first = filtered_chars.next().expect("Expected a digit.");

   filtered_chars 
        .last()
        .map_or_else(
            || format!("{first}{first}"),
            |last| format!("{first}{last}"),
        )
        .parse::<u32>()
        .expect("Expected a number.")
}
