//{"name":"A. Polycarp and Coins","group":"Codeforces - Codeforces Round 734 (Div. 3)","url":"https://codeforces.com/problemset/problem/1551/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1000\n30\n1\n32\n1000000000\n5\n","output":"334 333\n10 10\n1 0\n10 11\n333333334 333333333\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APolycarpAndCoins"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let mut c1 = n / 3;
    let mut c2 = n / 3;

    if n % 3 == 1 {
        c1 += 1;
    } else if n % 3 == 2{
        c2 += 1;
    }

    out.print_line((c1, c2));
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
        TaskType::Classic => {
            input.is_empty()
        }
        TaskType::Interactive => true,
    }
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
