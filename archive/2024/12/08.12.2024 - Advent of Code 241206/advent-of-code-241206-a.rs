//{"name":"advent-of-code-241206-a","group":"Advent of Code 241206","url":"https://adventofcode.com/2024/day/6","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241206-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_next_panel_data(
    grid: &Vec<Vec<char>>,
    pos_y: usize,
    pos_x: usize,
    direction: Direction,
) -> (bool, usize, usize, Direction) {
    match direction {
        Direction::Up => {
            if pos_y == 0 {
                return (false, 0, 0, Direction::Up);
            } else if grid[pos_y - 1][pos_x] == '#' {
                return get_next_panel_data(grid, pos_y, pos_x, Direction::Right);
            }
            (true, pos_y - 1, pos_x, Direction::Up)
        }
        Direction::Right => {
            if pos_x == grid[0].len() - 1 {
                return (false, 0, 0, Direction::Up);
            } else if grid[pos_y][pos_x + 1] == '#' {
                return get_next_panel_data(grid, pos_y, pos_x, Direction::Down);
            }
            (true, pos_y, pos_x + 1, Direction::Right)
        }
        Direction::Down => {
            if pos_y == grid.len() - 1 {
                return (false, 0, 0, Direction::Up);
            } else if grid[pos_y + 1][pos_x] == '#' {
                return get_next_panel_data(grid, pos_y, pos_x, Direction::Left);
            }
            (true, pos_y + 1, pos_x, Direction::Down)
        }
        Direction::Left => {
            if pos_x == 0 {
                return (false, 0, 0, Direction::Up);
            } else if grid[pos_y][pos_x - 1] == '#' {
                return get_next_panel_data(grid, pos_y, pos_x, Direction::Up);
            }
            (true, pos_y, pos_x - 1, Direction::Left)
        }
    }
}

fn traverse(
    grid: &mut Vec<Vec<char>>,
    pos_y: usize,
    pos_x: usize,
    direction: Direction,
    visited: &mut i32,
) {
    match grid[pos_y][pos_x] {
        '.' => {
            *visited += 1;
            grid[pos_y][pos_x] = '^'
        }
        '^' => (),
        '#' => panic!("traversed to obstacle ('#')!"),
        _ => unreachable!(),
    }

    let (next_possible, next_y, next_x, next_direction) =
        get_next_panel_data(&grid, pos_y, pos_x, direction);

    if !next_possible {
        return;
    }
    traverse(grid, next_y, next_x, next_direction, visited);
}

fn solve(input: &mut Input, output: &mut Output) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut pos_y: usize = 0;
    let mut pos_x: usize = 0;
    let mut visited: i32 = 1;

    while !input.is_empty() {
        grid.push(input.read_line().chars().collect());
    }

    'found: for test_y in 0..grid.len() {
        for test_x in 0..grid[0].len() {
            if grid[test_y][test_x] == '^' {
                pos_y = test_y;
                pos_x = test_x;
                break 'found;
            }
        }
    }

    traverse(&mut grid, pos_y, pos_x, Direction::Up, &mut visited);

    output.print_line(visited);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    input.is_empty()
}

//START MAIN
mod tester;
fn main() {
    tester::run_tests();
}
//END MAIN
