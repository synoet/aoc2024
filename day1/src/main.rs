fn part_one(input: String) -> i32 {
    let (mut left, mut right) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace().map(|i| i.parse::<i32>().unwrap());

            Some((nums.next().unwrap(), nums.next().unwrap()))
        })
        .unzip::<i32, i32, Vec<i32>, Vec<i32>>();

    left.sort();
    right.sort();

    let res = left
        .iter()
        .enumerate()
        .map(|(i, l)| (l - right[i]).abs() as i32)
        .sum::<i32>();

    res
}

fn part_two(input: String) -> i32 {
    let (mut left, mut right) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace().map(|i| i.parse::<i32>().unwrap());

            Some((nums.next().unwrap(), nums.next().unwrap()))
        })
        .unzip::<i32, i32, Vec<i32>, Vec<i32>>();

    let res = left
        .iter()
        .map(|l| l * right.iter().filter(|r| l == *r).count() as i32)
        .sum::<i32>();
    res
}
fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part_one(content.clone()));
    println!("Part 2: {}", part_two(content.clone()));
}
