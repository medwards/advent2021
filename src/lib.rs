use std::str::FromStr;

pub mod day_one;
pub mod day_three;
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
