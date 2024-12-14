//{"name":"advent-of-code-241214-b","group":"Advent of Code 241214","url":"https://adventofcode.com/2024/day/14","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241214-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use regex::Regex;
use std::io::stdin;

type Tiles = [[bool; 101]; 103];

struct Bot {
    py: usize,
    px: usize,
    vy: i32,
    vx: i32,
}

impl Bot {
    fn update_position(&mut self) {
        self.py = (self.py as i32 + self.vy).rem_euclid(103) as usize;
        self.px = (self.px as i32 + self.vx).rem_euclid(101) as usize;
    }
}

fn get_total_cluster(tiles: &Tiles) -> i32 {
    let mut cluster = 0;

    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            // if y > 0 &&
            if tiles[y][x] {
                if y > 0 && tiles[y - 1][x] {
                    cluster += 1;
                }
                if x > 0 && tiles[y][x - 1] {
                    cluster += 1;
                }
                if y + 1 < tiles.len() && tiles[y + 1][x] {
                    cluster += 1;
                }
                if x + 1 < tiles[0].len() && tiles[y][x + 1] {
                    cluster += 1;
                }
            }
        }
    }
    cluster
}

fn solve(input: &mut Input, _out: &mut Output) {
    let re = Regex::new(r"p=(?<px>\-?\d+),(?<py>\-?\d+) v=(?<vx>\-?\d+),(?<vy>\-?\d+)")
        .expect("regex invalid");

    let mut tiles: Tiles = [[false; 101]; 103];
    let mut iteration = 1;
    let mut bots: Vec<Bot> = vec![];
    let mut best_cluster = 0;

    while !input.is_empty() {
        let inpline = input.read_line();
        let caps = re.captures(&inpline).expect("caps invalid");

        let py = caps["py"].parse::<usize>().unwrap();
        let px = caps["px"].parse::<usize>().unwrap();
        let vy = caps["vy"].parse::<i32>().unwrap();
        let vx = caps["vx"].parse::<i32>().unwrap();

        bots.push(Bot { py, px, vy, vx });
    }

    loop {
        for bot in &mut bots {
            bot.update_position();
            tiles[bot.py][bot.px] = true;
        }
        let total_cluster = get_total_cluster(&tiles);

        if total_cluster >= best_cluster {
            best_cluster = total_cluster;
            dbg!(iteration, total_cluster);
            for y in 0..tiles.len() {
                for x in 0..tiles[0].len() {
                    if tiles[y][x] {
                        print!("⬜");
                    } else {
                        print!("⬛");
                    }
                }
                print!("\n");
            }
            let _ = stdin().read_line(&mut "".to_string()); // wait for user to enter for next iteration
        }
        iteration += 1;
        tiles = [[false; 101]; 103];
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

//START MAIN
mod tester;
fn main() {
    tester::run_tests();
}
//END MAIN
