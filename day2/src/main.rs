fn is_valid(a: i32, b: i32, increasing: bool) -> bool {
    if increasing {
        a < b && (a - b).abs() <= 3
    } else {
        a > b && (a - b).abs() <= 3
    }
}

fn is_list_valid(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let is_increasing = nums[0] < nums[1];

    for i in 0..nums.len() - 1 {
        if !is_valid(nums[i], nums[i + 1], is_increasing) {
            return false;
        }
    }
    true
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn part_one(sequences: &[Vec<i32>]) -> i32 {
    sequences.iter().filter(|nums| is_list_valid(nums)).count() as i32
}

fn part_two(sequences: &[Vec<i32>]) -> i32 {
    sequences
        .iter()
        .filter(|nums| {
            if is_list_valid(nums) {
                return true;
            }

            (0..nums.len()).any(|i| {
                let mut modified = nums.to_vec();
                modified.remove(i);
                is_list_valid(&modified)
            })
        })
        .count() as i32
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let sequences = parse_input(&input);

    println!("Part 1: {}", part_one(&sequences));
    println!("Part 2: {}", part_two(&sequences));
}
