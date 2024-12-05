/// Searching for patterns in a grid
///
/// e.g. find all `XMAS`
/// define a starting point, for `XMAS` `X` is a sufficient starting point
/// then define a pattern to build `XMAS` from moving on the grid from `X`
/// then explore the grid from `X` and count the number of times the pattern matches
///
/// https://adventofcode.com/2024/day/4

type Grid = Vec<Vec<char>>;
type Pattern = Vec<Coord>;
type Coord = (i32, i32);

fn find_starting_points(grid: &Grid, needle: &str) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == needle.chars().next().unwrap() {
                points.push((x as i32, y as i32));
            }
        }
    }
    points
}

fn in_bounds(grid: &Grid, x: i32, y: i32) -> bool {
    x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
}

fn explore(grid: &Grid, start: Coord, needle: &str, patterns: Vec<Pattern>) -> usize {
    let mut matches = 0;
    let needle_chars: Vec<char> = needle.chars().collect();

    for pattern in patterns {
        let mut found = true;
        for (i, (mx, my)) in pattern.iter().enumerate() {
            let new_x = start.0 + mx;
            let new_y = start.1 + my;

            if !in_bounds(grid, new_x, new_y) {
                found = false;
                break;
            }

            let content = grid[new_x as usize][new_y as usize];
            if content != needle_chars[i] {
                found = false;
                break;
            }
        }
        if found {
            matches += 1;
        }
    }
    matches
}

const PART_ONE_PATTERN: [[Coord; 4]; 8] = [
    // straight across
    [(0, 0), (1, 0), (2, 0), (3, 0)],
    // straight down
    [(0, 0), (0, 1), (0, 2), (0, 3)],
    // straight up
    [(0, 0), (0, -1), (0, -2), (0, -3)],
    // straight left
    [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
    // diagonal top right
    [(0, 0), (1, -1), (2, -2), (3, -3)],
    // diagonal top left
    [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
    // diagonal bottom right
    [(0, 0), (1, 1), (2, 2), (3, 3)],
    // diagonal bottom left
    [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
];

const PART_TWO_PATTERN: [[Coord; 5]; 4] = [
    // MSAMS
    [(-1, -1), (1, -1), (0, 0), (-1, 1), (1, 1)],
    // SMASM
    [(1, -1), (-1, -1), (0, 0), (1, 1), (-1, 1)],
    // SSAMM
    [(1, 1), (1, -1), (0, 0), (-1, 1), (-1, -1)],
    // MMASS
    [(1, -1), (1, 1), (0, 0), (-1, -1), (-1, 1)],
];

fn part_one(grid: &Grid) -> usize {
    let starting_points = find_starting_points(grid, "XMAS");
    let mut count = 0;
    for point in starting_points {
        count += explore(
            grid,
            point,
            "XMAS",
            PART_ONE_PATTERN
                .clone()
                .into_iter()
                .map(|p| p.to_vec())
                .collect(),
        );
    }
    count
}

fn part_two(grid: &Grid) -> usize {
    let starting_points = find_starting_points(grid, "A");
    let mut count = 0;
    for point in starting_points {
        count += explore(
            grid,
            point,
            "MSAMS",
            PART_TWO_PATTERN
                .clone()
                .into_iter()
                .map(|p| p.to_vec())
                .collect(),
        );
    }
    count
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let grid = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part one: {}", part_one(&grid));
    println!("Part two: {}", part_two(&grid));
}
