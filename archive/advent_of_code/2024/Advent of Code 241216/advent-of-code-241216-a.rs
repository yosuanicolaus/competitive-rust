//{"name":"advent-of-code-241216-a","group":"Advent of Code 241216","url":"https://adventofcode.com/2024/day/16","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241216-a"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn get_dir_idx(dir: Direction) -> usize {
    match dir {
        Direction::Up => 0,
        Direction::Left => 1,
        Direction::Down => 2,
        Direction::Right => 3,
    }
}

fn get_next_cost(curr_cost: i32, curr_dir: Direction, next_dir: Direction) -> Reverse<i32> {
    if curr_dir == next_dir {
        Reverse(curr_cost + 1)
    } else {
        Reverse(curr_cost + 1001)
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut costs: Vec<Vec<[i32; 4]>> = vec![];

    while !input.is_empty() {
        grid.push(input.read_line().chars().collect());
        let gl = grid[0].len();
        costs.push(vec![[i32::MAX; 4]; gl]);
    }

    let mut pq: BinaryHeap<(Reverse<i32>, usize, usize, Direction)> =
        BinaryHeap::from([(Reverse(0), grid.len() - 2, 1, Direction::Right)]);

    while let Some((Reverse(cost), y, x, dir)) = pq.pop() {
        if cost >= costs[y][x][get_dir_idx(dir)] {
            continue;
        }
        costs[y][x][get_dir_idx(dir)] = cost;

        if grid[y][x] == 'E' {
            out.println(cost);
            return;
        }
        if grid[y - 1][x] != '#' {
            let next_cost = get_next_cost(cost, dir, Direction::Up);
            pq.push((next_cost, y - 1, x, Direction::Up));
        }
        if grid[y][x - 1] != '#' {
            let next_cost = get_next_cost(cost, dir, Direction::Left);
            pq.push((next_cost, y, x - 1, Direction::Left));
        }
        if grid[y + 1][x] != '#' {
            let next_cost = get_next_cost(cost, dir, Direction::Down);
            pq.push((next_cost, y + 1, x, Direction::Down));
        }
        if grid[y][x + 1] != '#' {
            let next_cost = get_next_cost(cost, dir, Direction::Right);
            pq.push((next_cost, y, x + 1, Direction::Right))
        }
    }
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
