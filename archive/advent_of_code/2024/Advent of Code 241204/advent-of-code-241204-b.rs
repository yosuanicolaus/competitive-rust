//{"name":"advent-of-code-241204-b","group":"Advent of Code 241204","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241204-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::HashSet;

type PreCalc = HashSet<&'static str>;

fn check_and_add(
    board: &Vec<Vec<char>>,
    start_y: usize,
    start_x: usize,
    valid_count: &mut i32,
    valid_patterns: &HashSet<&str>,
) {
    let word = [
        board[start_y][start_x],
        board[start_y][start_x + 2],
        board[start_y + 1][start_x + 1],
        board[start_y + 2][start_x],
        board[start_y + 2][start_x + 2],
    ]
    .iter()
    .collect::<String>();

    if valid_patterns.contains(word.as_str()) {
        *valid_count += 1;
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, valid_patterns: &mut PreCalc) {
    let mut board: Vec<Vec<char>> = vec![];
    let mut valid_count = 0;

    while !input.is_empty() {
        board.push(input.read_line().chars().collect());
    }

    let len_y = board.len();
    let len_x = board[0].len();

    for y in 0..len_y - 2 {
        for x in 0..len_x - 2 {
            check_and_add(&board, y, x, &mut valid_count, &valid_patterns);
        }
    }

    out.print_line(valid_count);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = HashSet::from(["MMASS", "SMASM", "SSAMM", "MSAMS"]);

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
