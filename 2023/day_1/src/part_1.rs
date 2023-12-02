use std::str::Chars;

fn main() {
    let input = include_str!("input_1.txt");
    let result = part_1(input);
    println!("Result: {}", result);
}

fn part_1(input: &str) -> i32 {
    let mut total: i32 = 0;

    input.lines().for_each(|x| {
        let line_parsed = x.parse::<String>().unwrap();
        let first_number = find_first_number_in_chars(&mut line_parsed.chars());
        let final_number =
            find_first_number_in_chars(&mut line_parsed.chars().rev().collect::<String>().chars());

        let full_number = format!("{}{}", first_number.unwrap(), final_number.unwrap())
            .parse::<i32>()
            .unwrap();

        total += full_number;
    });

    total
}

fn find_first_number_in_chars(chars: &mut Chars) -> Option<String> {
    Some(
        chars
            .find(|&x| {
                let parsed = x.to_string().parse::<i32>();
                if parsed.is_ok() {
                    return true;
                }

                false
            })?
            .to_string()
            .parse::<i32>()
            .ok()?
            .to_string(),
    )
}
