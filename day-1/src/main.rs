use std::fs;

const FILE_PATH: &str = "src/pairs.txt";

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let sum: i32 = contents
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(first), Some(second)) => {
                    let parsed_first = first.parse::<i32>();
                    let parsed_second = second.parse::<i32>();
                    match (parsed_first, parsed_second) {
                        (Ok(x), Ok(y)) => Some((x - y).abs()),
                        _ => None,
                    }
                }
                _ => None,
            }
        })
        .sum();

    println!("The sum is {sum}");
}
