//{"name":"A. Arrival of the General","group":"Codeforces - Codeforces Round 103 (Div. 2)","url":"https://codeforces.com/problemset/problem/144/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n33 44 11 22\n","output":"2\n"},{"input":"7\n10 10 58 31 63 40 76\n","output":"10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AArrivalOfTheGeneral"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut lo = 100;
    let mut hi = 1;
    let mut loidx = 0usize;
    let mut hiidx = 0usize;

    for (i, &num) in a.iter().enumerate() {
        if num > hi {
            hi = num;
            hiidx = i;
        }
        if num <= lo {
            lo = num;
            loidx = i;
        }
    }

    let mut ans = (n - 1) - loidx + hiidx;
    if loidx < hiidx {
        ans -= 1;
    }
    out.println(ans);
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
