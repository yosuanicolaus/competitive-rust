//{"name":"A. Tender Carpenter","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\n2 3 5 7\n4\n115 9 2 28\n5\n8 4 1 6 2\n6\n1 5 4 1 4 7\n2\n100000 100000\n","output":"YES\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATenderCarpenter"}}}

use std::cmp;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = vec![];

    for _ in 0..n {
        a.push(input.read_int());
    }

    for i in 0..n - 1 {
        let j = i + 1;
        let (c, d) = (cmp::min(a[i], a[j]), cmp::max(a[i], a[j]));
        if c * 2 > d {
            out.println("YES");
            return;
        }
    }
    out.println("NO");
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
    tester::run_main();
}
//END MAIN
