//{"name":"B - Vertical Reading","group":"AtCoder - AtCoder Beginner Contest 360","url":"https://atcoder.jp/contests/abc360/tasks/abc360_b","interactive":false,"timeLimit":2000,"tests":[{"input":"atcoder toe\n","output":"Yes\n"},{"input":"beginner r\n","output":"No\n"},{"input":"verticalreading agh\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVerticalReading"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (s, t) = input
        .read_line()
        .split_once(' ')
        .map(|(a, b)| (a.to_owned(), b.to_owned()))
        .unwrap();
    let svc: Vec<char> = s.chars().collect();
    let mut tvc: Vec<char> = Vec::new();

    for interval in 1..svc.len() {
        for start_idx in 0..interval {
            let mut i = start_idx;

            while i < svc.len() {
                tvc.push(svc[i]);
                i += interval;
            }

            let test_t: String = tvc.iter().collect();
            if t == test_t {
                out.print_line("Yes");
                return;
            }
            tvc.clear();
        }
    }
    out.print_line("No");
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
