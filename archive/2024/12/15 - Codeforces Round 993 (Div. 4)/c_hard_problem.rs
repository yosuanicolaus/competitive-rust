//{"name":"C. Hard Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 5 5 10\n3 6 1 1\n15 14 12 4\n1 1 1 1\n420 6 9 69\n","output":"20\n5\n30\n2\n84\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CHardProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read_long();
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();

    let mut free = 0;
    let mut seated = 0;

    if a > m {
        seated += m;
    } else {
        seated += a;
        free += m - a;
    }

    if b > m {
        seated += m;
    } else {
        seated += b;
        free += m - b;
    }

    if c > free {
        seated += free;
    } else {
        seated += c;
    }

    out.println(seated);
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
