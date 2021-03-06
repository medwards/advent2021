use std::str::FromStr;

pub mod day_eight;
pub mod day_eleven;
pub mod day_fifteen;
pub mod day_five;
pub mod day_four;
pub mod day_fourteen;
pub mod day_nine;
pub mod day_one;
pub mod day_seven;
pub mod day_six;
pub mod day_six_jhorstmann;
pub mod day_sixteen;
pub mod day_ten;
pub mod day_thirteen;
pub mod day_three;
pub mod day_twelve;
pub mod day_two;

#[derive(Debug)]
pub enum Error {
    InvalidDay,
}

pub fn get_day(
    day: &str,
) -> Result<
    (
        &str,
        &str,
        fn(&str) -> anyhow::Result<usize>,
        fn(&str) -> anyhow::Result<usize>,
    ),
    Error,
> {
    match day {
        "1" | "one" => Ok((
            "1",
            day_one::INPUT_PATH,
            day_one::part_one,
            day_one::part_two,
        )),
        "2" | "two" => Ok((
            "2",
            day_two::INPUT_PATH,
            day_two::part_one,
            day_two::part_two,
        )),
        "3" | "three" => Ok((
            "3",
            day_three::INPUT_PATH,
            day_three::part_one,
            day_three::part_two,
        )),
        "4" | "four" => Ok((
            "4",
            day_four::INPUT_PATH,
            day_four::part_one,
            day_four::part_two,
        )),
        "5" | "five" => Ok((
            "5",
            day_five::INPUT_PATH,
            day_five::part_one,
            day_five::part_two,
        )),
        "6" | "six" => Ok((
            "6",
            day_six::INPUT_PATH,
            day_six::part_one,
            day_six::part_two,
        )),
        "jhorstmann::6" | "jhorstmann::six" => Ok((
            "jhorstmann::6",
            day_six_jhorstmann::INPUT_PATH,
            day_six_jhorstmann::part_one,
            day_six_jhorstmann::part_two,
        )),
        "7" | "seven" => Ok((
            "7",
            day_seven::INPUT_PATH,
            day_seven::part_one,
            day_seven::part_two,
        )),
        "8" | "eight" => Ok((
            "8",
            day_eight::INPUT_PATH,
            day_eight::part_one,
            day_eight::part_two,
        )),
        "9" | "nine" => Ok((
            "9",
            day_nine::INPUT_PATH,
            day_nine::part_one,
            day_nine::part_two,
        )),
        "10" | "ten" => Ok((
            "10",
            day_ten::INPUT_PATH,
            day_ten::part_one,
            day_ten::part_two,
        )),
        "11" | "eleven" => Ok((
            "11",
            day_eleven::INPUT_PATH,
            day_eleven::part_one,
            day_eleven::part_two,
        )),
        "12" | "twelve" => Ok((
            "12",
            day_twelve::INPUT_PATH,
            day_twelve::part_one,
            day_twelve::part_two,
        )),
        "13" | "thirteen" => Ok((
            "13",
            day_thirteen::INPUT_PATH,
            day_thirteen::part_one,
            day_thirteen::part_two,
        )),
        "14" | "fourteen" => Ok((
            "14",
            day_fourteen::INPUT_PATH,
            day_fourteen::part_one,
            day_fourteen::part_two,
        )),
        "15" | "fifteen" => Ok((
            "15",
            day_fifteen::INPUT_PATH,
            day_fifteen::part_one,
            day_fifteen::part_two,
        )),
        "16" | "sixteen" => Ok((
            "16",
            day_sixteen::INPUT_PATH,
            day_sixteen::part_one,
            day_sixteen::part_two,
        )),
        _ => Err(Error::InvalidDay),
    }
}

pub fn load_integers(contents: &str) -> anyhow::Result<Vec<usize>> {
    let integers: anyhow::Result<Vec<_>> = contents
        .lines()
        .map(|s| s.parse().map_err(anyhow::Error::new))
        .collect();
    integers
}

pub fn read_to_lines(contents: &str) -> anyhow::Result<Vec<String>> {
    let lines: Vec<_> = contents
        .lines()
        .flat_map(String::from_str) // can't fail
        .collect();
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::load_integers;

    #[test]
    fn test_load_integers() {
        let expected_output = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(
            expected_output,
            load_integers(
                read_to_string("fixtures/positive_integers.txt")
                    .expect("missing fixture")
                    .as_ref()
            )
            .expect("Unexpected failure")
        );
    }
}
