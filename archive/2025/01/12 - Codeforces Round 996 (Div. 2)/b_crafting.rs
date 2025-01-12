//{"name":"B. Crafting","group":"Codeforces - Codeforces Round 996 (Div. 2)","url":"https://codeforces.com/contest/2055/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n0 5 5 1\n1 4 4 0\n3\n1 1 3\n2 2 1\n2\n1 10\n3 3\n","output":"YES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCrafting"}}}

use std::cmp;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_vec_int(n);
    let b = input.read_vec_int(n);

    let mut smallest_tres = i32::MAX;
    let mut need_opr = 0;
    let mut need_val = 0;

    for i in 0..n {
        if a[i] < b[i] {
            need_opr += 1;
            need_val = b[i] - a[i];
        } else {
            smallest_tres = cmp::min(smallest_tres, a[i] - b[i]);
        }
    }

    if need_opr > 1 {
        out.println("NO");
    } else if need_opr == 0 {
        out.println("YES");
    } else {
        // need_opr == 1, need_val is valid
        if need_val <= smallest_tres {
            out.println("YES");
        } else {
            out.println("NO");
        }
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
    tester::run_main();
}
//END MAIN
