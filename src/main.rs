use std::{fs::read_to_string, str::FromStr};

use anyhow::Result;

mod day_one;
mod day_three;
mod day_two;

fn main() {
    let mut app = clap::App::new("Advent of Code 2021 Solver")
        .author("Michael Edwards <medwards@walledcity.ca>")
        .subcommand(
            clap::App::new("day").about("Select day to solve").arg(
                clap::Arg::new("DAY")
                    .multiple_occurrences(true)
                    .min_values(1)
                    .required(true),
            ),
        );
    let subcommand_error = app.error(
        clap::ErrorKind::MissingSubcommand,
        "Missing subcommand which wasn't expected. Did you mean 'day'?",
    );
    let invalid_day_error = app.error(
        clap::ErrorKind::InvalidValue,
        "a DAY argument wasn't recognized",
    );
    let matches = app.get_matches();

    let days = if let Some(subcommand) = matches.subcommand_matches("day") {
        subcommand.values_of("DAY").expect("day was not provided")
    } else {
        subcommand_error.exit();
    };

    days.for_each(|day| {
        let (day, input_path, part_one, part_two): (
            &str,
            &str,
            fn(&str) -> Result<usize>,
            fn(&str) -> Result<usize>,
        ) = match day {
            "1" | "one" => (
                "1",
                day_one::INPUT_PATH,
                day_one::part_one,
                day_one::part_two,
            ),
            "2" | "two" => (
                "2",
                day_two::INPUT_PATH,
                day_two::part_one,
                day_two::part_two,
            ),
            "3" | "three" => (
                "3",
                day_three::INPUT_PATH,
                day_three::part_one,
                day_three::part_two,
            ),
            _ => invalid_day_error.exit(),
        };

        println!("Day {}, Part One: {}", day, part_one(input_path).unwrap());
        println!("Day {}, Part One: {}", day, part_two(input_path).unwrap());
    });
}

fn load_integers(path: &str) -> Result<Vec<usize>> {
    let contents = read_to_string(path)?;
    let integers: Result<Vec<_>> = contents
        .trim()
        .split('\n')
        .map(|s| s.parse().map_err(anyhow::Error::new))
        .collect();
    integers
}

fn read_to_lines(path: &str) -> Result<Vec<String>> {
    let contents = read_to_string(path)?;
    let lines: Vec<_> = contents
        .trim()
        .split('\n')
        .flat_map(String::from_str) // can't fail
        .collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::load_integers;

    #[test]
    fn test_load_integers() {
        let expected_output = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(
            expected_output,
            load_integers("fixtures/positive_integers.txt").expect("Unexpected failure")
        );
    }
}
