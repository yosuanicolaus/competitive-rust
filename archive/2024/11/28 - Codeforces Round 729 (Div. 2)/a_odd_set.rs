//{"name":"A. Odd Set","group":"Codeforces - Codeforces Round 729 (Div. 2)","url":"https://codeforces.com/problemset/problem/1542/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n2 3 4 5\n3\n2 3 4 5 5 5\n1\n2 4\n1\n2 3\n4\n1 5 3 2 6 7 3 4\n","output":"Yes\nNo\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AOddSet"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(2 * n);

    let mut total_odd = 0;
    let mut total_even = 0;

    for num in a {
        if num % 2 == 1 {
            total_odd += 1;
        } else {
            total_even += 1;
        }
    }

    if total_odd == total_even {
        out.print_line("Yes");
    } else {
        out.print_line("No");
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
