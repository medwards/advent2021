use std::fs::read_to_string;

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
        let (day, input_path, part_one, part_two) =
            advent2021::get_day(day).unwrap_or_else(|_| invalid_day_error.exit());

        let contents = read_to_string(input_path)
            .unwrap_or_else(|e| panic!("Unable to read from {} - {}", input_path, e));

        println!(
            "Day {}, Part One: {}",
            day,
            part_one(contents.as_str()).unwrap()
        );
        println!(
            "Day {}, Part One: {}",
            day,
            part_two(contents.as_str()).unwrap()
        );
    });
}
