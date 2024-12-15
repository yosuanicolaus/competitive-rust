//{"name":"advent-of-code-241215-a","group":"Advent of Code 241215","url":"https://adventofcode.com/2024/day/15","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241215-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn get_direction_y_x(m: char) -> (isize, isize) {
    match m {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unreachable!(),
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];
    let mut by = 1usize;
    let mut bx = 1usize;

    while !input.is_empty() {
        let mut line: Vec<char> = input.read_line().chars().collect();

        if line[0] == '#' {
            // input first phase
            grid.push(line);
        } else {
            // end of first phase
            moves.append(&mut line);
        }
    }

    // get bot position
    'bot_pos: for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                by = y;
                bx = x;
                break 'bot_pos;
            }
        }
    }

    for &m in &moves {
        let (dy, dx) = get_direction_y_x(m);
        let mut ny = by.wrapping_add_signed(dy);
        let mut nx = bx.wrapping_add_signed(dx);

        while grid[ny][nx] == 'O' {
            ny = ny.wrapping_add_signed(dy);
            nx = nx.wrapping_add_signed(dx);
        }

        match grid[ny][nx] {
            '.' => {
                let mut ty = ny;
                let mut tx = nx;
                loop {
                    ny = ny.wrapping_add_signed(-dy);
                    nx = nx.wrapping_add_signed(-dx);
                    grid[ty][tx] = grid[ny][nx];

                    if grid[ty][tx] == '@' {
                        grid[ny][nx] = '.';
                        by = ty;
                        bx = tx;
                        break;
                    }
                    ty = ny;
                    tx = nx;
                }
            }
            '#' => {}
            _ => {
                dbg!(grid[ny][nx], ny, nx);
                unreachable!();
            }
        }
    }

    let mut ans = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            if grid[y][x] == 'O' {
                ans += 100 * y + x;
            }
        }
    }

    out.println(ans);
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
