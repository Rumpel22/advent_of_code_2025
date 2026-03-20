use std::str::FromStr;

use nom::{
    Finish, IResult, Parser,
    branch::alt,
    character::{char, complete},
    combinator::value,
    error::Error,
    multi::{many1, separated_list1},
    sequence::delimited,
};

use crate::{Buttons, Joltages, Led, Machine};

fn parse_leds(input: &str) -> IResult<&str, Vec<Led>> {
    delimited(
        char('['),
        many1(alt((value(Led::Off, char('.')), value(Led::On, char('#'))))),
        char(']'),
    )
    .parse(input)
}

fn parse_button(input: &str) -> IResult<&str, Vec<u32>> {
    delimited(
        char('('),
        separated_list1(char(','), complete::u32),
        char(')'),
    )
    .parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Buttons> {
    let (input, data) = separated_list1(char(' '), parse_button).parse(input)?;

    let nb_buttons = data.len();
    let nb_leds = *data
        .iter()
        .map(|column| column.iter().max().unwrap())
        .max()
        .unwrap() as usize
        + 1;
    let mut v = Vec::new();
    v.resize(nb_buttons * nb_leds, 0);

    data.iter().enumerate().for_each(|(c_index, c)| {
        c.iter().for_each(|element| {
            let index = c_index * nb_leds + *element as usize;
            v[index] = 1_u32;
        })
    });
    let matrix = Buttons::from_vec(nb_leds, nb_buttons, v);

    Ok((input, matrix))
}

fn parse_joltages(input: &str) -> IResult<&str, Joltages> {
    let (input, data) = delimited(
        char('{'),
        separated_list1(char(','), complete::u32),
        char('}'),
    )
    .parse(input)?;

    Ok((input, Joltages::from_vec(data)))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, leds) = parse_leds(input)?;
    let (input, _) = char(' ').parse(input)?;
    let (input, buttons) = parse_buttons(input)?;
    let (input, _) = char(' ').parse(input)?;
    let (input, joltages) = parse_joltages(input)?;
    Ok((
        input,
        Machine {
            leds,
            buttons,
            joltages,
        },
    ))
}

impl FromStr for Machine {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match parse_machine(input).finish() {
            Ok((_, machine)) => Ok(machine),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}
