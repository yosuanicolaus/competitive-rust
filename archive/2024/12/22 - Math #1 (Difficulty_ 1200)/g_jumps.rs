//{"name":"G. Jumps","group":"Codeforces - Math #1 (Difficulty: 1200)","url":"https://codeforces.com/group/JESCgZZ8qn/contest/333990/problem/G","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n2\n3\n4\n5\n","output":"1\n3\n2\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GJumps"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// with editorial help ._.

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_int();

    let mut n = ((2 * x) as f64).sqrt() as i32;
    let mut point = n * (n + 1) / 2;
    while point < x {
        n += 1;
        point = n * (n + 1) / 2;
    }

    // it's proven that any +2 or more overlap can be offset by stepping backwards
    // at some time(s) before reaching >= of the point
    // except when it's +1 overlap (must step back at the very end)
    if point == x + 1 {
        n += 1;
    }
    out.println(n);
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
