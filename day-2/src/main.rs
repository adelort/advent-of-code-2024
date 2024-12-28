use std::fs;

const FILE_PATH: &str = "src/reports.txt";

struct Report {
    does_increase: bool,
    does_decrease: bool,
    respects_limits: bool,
    last_value: Option<i32>,
}

impl Report {
    fn new() -> Report {
        Report {
            does_increase: true,
            does_decrease: true,
            respects_limits: true,
            last_value: None,
        }
    }

    fn process_level(&mut self, level: i32) {
        if let Some(last_value) = self.last_value {
            if level > last_value {
                self.does_decrease = false;
            }
            if level < last_value {
                self.does_increase = false;
            }
            let diff = (level - last_value).abs();
            if diff < 1 || diff > 3 {
                self.respects_limits = false;
            }
        }

        self.last_value = Some(level);
    }

    fn is_safe(&self) -> bool {
        (self.does_increase || self.does_decrease) && self.respects_limits
    }
}

fn main() {
    let contents = fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

    let safe_report_count = contents
        .lines()
        .filter(|line| {
            let mut report = Report::new();

            line.split_whitespace()
                .filter_map(|level| level.parse::<i32>().ok())
                .for_each(|level| report.process_level(level));

            report.is_safe()
        })
        .count();

    println!("There are {safe_report_count} safe reports");
}
