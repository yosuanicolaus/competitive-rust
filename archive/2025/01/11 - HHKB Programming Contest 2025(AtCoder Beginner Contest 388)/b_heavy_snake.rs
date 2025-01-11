//{"name":"B - Heavy Snake","group":"AtCoder - HHKB Programming Contest 2025(AtCoder Beginner Contest 388)","url":"https://atcoder.jp/contests/abc388/tasks/abc388_b","interactive":false,"timeLimit":2000,"tests":[{"input":"4 3\n3 3\n5 1\n2 4\n1 10\n","output":"12\n15\n20\n"},{"input":"1 4\n100 100\n","output":"10100\n10200\n10300\n10400\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHeavySnake"}}}

use std::cmp;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let d = input.read_int();
    let mut ts = vec![];
    let mut ls = vec![];

    for _ in 0..n {
        ts.push(input.read_int());
        ls.push(input.read_int());
    }

    for k in 1..=d {
        let mut m = 0;
        for i in 0..n {
            m = cmp::max(m, (k + ls[i]) * ts[i]);
        }
        out.println(m);
    }
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
    tester::run_main();
}
//END MAIN
