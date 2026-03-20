use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_until},
    character::{
        char,
        complete::{digit1, newline, space1, usize},
    },
    combinator::opt,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
};

use crate::{Region, Shape};

fn parse_shape(input: &str) -> IResult<&str, Shape> {
    let (input, _) = (digit1, tag(":\n")).parse(input)?;
    terminated(take_until("\n\n"), tag("\n\n"))
        .map(|shape: &str| {
            let fields = shape
                .lines()
                .flat_map(|line| {
                    line.chars().map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => unreachable!(),
                    })
                })
                .collect::<Vec<_>>();
            Shape { fields }
        })
        .parse(input)
}

fn parse_shapes(input: &str) -> IResult<&str, Vec<Shape>> {
    many1(parse_shape).parse(input)
}

fn parse_region(input: &str) -> IResult<&str, Region> {
    let (input, (x, y)) = separated_pair(usize, char('x'), usize).parse(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, presents) =
        terminated(separated_list1(space1, usize), opt(newline)).parse(input)?;

    Ok((input, Region { x, y, presents }))
}

fn parse_regions(input: &str) -> IResult<&str, Vec<Region>> {
    many1(parse_region).parse(input)
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Shape>, Vec<Region>)> {
    let (input, shapes) = parse_shapes(input)?;
    let (input, regions) = parse_regions(input)?;
    Ok((input, (shapes, regions)))
}

pub fn parse(input: &str) -> (Vec<Shape>, Vec<Region>) {
    match parse_input(input) {
        Ok((_, (shapes, regions))) => (shapes, regions),
        Err(error) => unreachable!("{}", error.to_string()),
    }
}
