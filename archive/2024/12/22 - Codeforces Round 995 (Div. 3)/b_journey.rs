//{"name":"B. Journey","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n12 1 5 3\n6 6 7 4\n16 3 4 1\n1000000000 1 1 1\n","output":"5\n1\n6\n1000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BJourney"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut n: i64 = input.read();
    let a: i64 = input.read();
    let b: i64 = input.read();
    let c: i64 = input.read();
    let tot = a + b + c;

    let mut days = 0;
    if n / tot > 0 {
        let tot_cyc = n / tot;
        days += tot_cyc * 3;
        n -= tot * tot_cyc;
    }

    let mut cyc = 0;
    while n > 0 {
        if cyc == 0 {
            n -= a;
            cyc = 1;
        } else if cyc == 1 {
            n -= b;
            cyc = 2;
        } else {
            n -= c;
            cyc = 0;
        }
        days += 1;
    }

    out.println(days);
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
