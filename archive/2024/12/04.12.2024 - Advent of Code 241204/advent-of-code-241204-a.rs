//{"name":"advent-of-code-241204-a","group":"Advent of Code 241204","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241204-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn check_and_add(word: &str, valid_count: &mut i32) {
    if word == "XMAS" {
        *valid_count += 1;
    }
    if word == "SAMX" {
        *valid_count += 1;
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut board: Vec<Vec<char>> = vec![];
    let mut valid_words = 0;

    while !input.is_empty() {
        board.push(input.read_line().chars().collect());
    }

    let len_y = board.len();
    let len_x = board[0].len();

    for y in 0..len_y {
        for x in 0..len_x {
            // check horizontal
            if x + 3 < len_x {
                let word = board[y][x..x + 4].iter().collect::<String>();
                check_and_add(&word, &mut valid_words);
            }

            // check vertical
            if y + 3 < len_y {
                let mut word = String::from(board[y][x]);
                for ni in 1..=3 {
                    word.push(board[y + ni][x]);
                }
                check_and_add(&word, &mut valid_words);
            }

            // check diagonal down right
            if x + 3 < len_x && y + 3 < len_y {
                let mut word = String::from(board[y][x]);
                for ni in 1..=3 {
                    word.push(board[y + ni][x + ni]);
                }
                check_and_add(&word, &mut valid_words);
            }

            // check diagonal up right
            if x + 3 < len_x && y >= 3 {
                let mut word = String::from(board[y][x]);
                for ni in 1..=3 {
                    word.push(board[y - ni][x + ni]);
                }
                check_and_add(&word, &mut valid_words);
            }
        }
    }

    out.print_line(valid_words);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
