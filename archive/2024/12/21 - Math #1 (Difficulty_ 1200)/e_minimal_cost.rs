//{"name":"E. Minimal Cost","group":"Codeforces - Math #1 (Difficulty: 1200)","url":"https://codeforces.com/group/JESCgZZ8qn/contest/333990/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3 4\n2 2\n2 3 4\n3 2\n2 4 3\n3 2\n","output":"7\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMinimalCost"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let u: i32 = input.read();
    let v: i32 = input.read();
    let mut maxdiff = 0;
    let mut last = i32::MAX;

    for _ in 0..n {
        let obs = input.read_int();
        if last == i32::MAX {
            last = obs;
        } else {
            maxdiff = cmp::max(maxdiff, (last - obs).abs());
            last = obs;
        }
    }

    if maxdiff == 0 {
        out.println(cmp::min(2 * v, u + v));
    } else if maxdiff == 1 {
        out.println(cmp::min(u, v));
    } else {
        out.println(0);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
