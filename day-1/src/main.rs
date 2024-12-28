use std::{collections::HashMap, fs};

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

    let mut similarity_score = 0;

    let key_count: HashMap<i32, i32> = contents.lines().fold(HashMap::new(), |mut acc, line| {
        let x = line
            .split_whitespace()
            .nth(1)
            .and_then(|item| item.parse::<i32>().ok())
            .unwrap_or(0);

        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    contents.lines().for_each(|line| {
        let parsed_first = line
            .split_whitespace()
            .next()
            .and_then(|x| x.parse::<i32>().ok());

        if let Some(parsed) = parsed_first {
            similarity_score += parsed * *key_count.get(&parsed).unwrap_or(&0);
        }
    });

    println!("The similarity score is {similarity_score}");
}
