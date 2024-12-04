use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

// Define our AST node
#[derive(Debug, PartialEq, Clone, Copy)]
struct Multiply {
    left: usize,
    right: usize,
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn multiplication(input: &str) -> IResult<&str, Multiply> {
    let (remaining, (_, _, left, _, right, _)) = tuple((
        tag("mul"),
        tag("("),
        number,
        tuple((multispace0, tag(","), multispace0)),
        number,
        tag(")"),
    ))(input)?;

    Ok((remaining, Multiply { left, right }))
}

fn between_dont_do(input: &str) -> IResult<&str, &str> {
    let (input_after_dont, _) = take_until("don't()")(input)?;
    let (input_after_tag, _) = tag("don't()")(input_after_dont)?;
    let (input_after_do, _) = take_until("do()")(input_after_tag)?;
    let (remaining, _) = tag("do()")(input_after_do)?;
    Ok((
        remaining,
        &input_after_dont[..input_after_dont.len() - remaining.len()],
    ))
}

fn between_dont_end(input: &str) -> IResult<&str, &str> {
    let (input_after_dont, _) = take_until("don't()")(input)?;
    let (remaining, _) = tag("don't()")(input_after_dont)?;
    Ok((remaining, &input_after_dont[..]))
}

fn find_dont(input: &str) -> IResult<&str, &str> {
    alt((between_dont_do, between_dont_end))(input)
}

fn remove_dont(input: &str) -> IResult<&str, String> {
    let mut new = input.to_string();
    while let Ok((_, donot)) = find_dont(&new) {
        if donot.is_empty() {
            break;
        }
        new = new.replace(donot, "");
    }
    Ok((input, new))
}

fn find_multiply(input: &str) -> IResult<&str, Option<Multiply>> {
    let (remaining, _) = take_until("mul")(input)?;
    let (remaining_skip, _) = tag("mul")(remaining)?;

    match multiplication(remaining) {
        Ok((remaining, multiply)) => Ok((remaining, Some(multiply))),
        Err(_) => Ok((remaining_skip, None)),
    }
}

fn find_all_multiply(input: &str) -> IResult<&str, Vec<Multiply>> {
    let mut current = input;
    let mut results = vec![];

    while !current.is_empty() {
        match find_multiply(current) {
            Ok((remaining, multiply)) => {
                if let Some(multiply) = multiply {
                    current = remaining;
                    results.push(multiply);
                } else {
                    current = remaining;
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok((input, results))
}

fn part_one(input: &str) -> usize {
    find_all_multiply(&input)
        .unwrap()
        .1
        .iter()
        .map(|multiply| multiply.left * multiply.right)
        .sum()
}

fn part_two(input: &str) -> usize {
    let current = remove_dont(input)
        .and_then(|(_, s)| Ok(s.to_string()))
        .unwrap_or(input.to_string());

    find_all_multiply(&current)
        .unwrap()
        .1
        .iter()
        .map(|multiply| multiply.left * multiply.right)
        .sum()
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    println!("{:?}", part_one(&content));
    println!("{:?}", part_two(&content));
}
