//{"name":"C. Isamatdin and His Magic Wand!","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/problemset/problem/2167/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4\n2 3 1 4\n5\n3 2 1 3 4\n4\n3 7 5 1\n2\n1000000000 2\n3\n1 3 5\n5\n2 5 3 1 7\n4\n2 4 8 6\n","output":"1 2 3 4\n1 2 3 3 4\n3 7 5 1\n1000000000 2\n1 3 5\n1 2 3 5 7\n2 4 8 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIsamatdinAndHisMagicWand"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = input.read_vec_int(n);
    let mut have_odd = false;
    let mut have_even = false;

    for num in &a {
        if num % 2 == 0 {
            have_even = true;
        } else {
            have_odd = true;
        }
        if have_even && have_odd {
            break;
        }
    }

    if have_even && have_odd {
        a.sort();
    }
    out.println(a);
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
    tester::run_main();
}
//END MAIN
