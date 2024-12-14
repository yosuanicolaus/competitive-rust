//{"name":"A. Bus to Pénjamo","group":"Codeforces - Codeforces Round 978 (Div. 2)","url":"https://codeforces.com/problemset/problem/2022/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 3\n2 3 1\n3 3\n2 2 2\n4 5\n1 1 2 2\n4 5\n3 1 1 3\n","output":"4\n6\n6\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ABusToPnjamo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let r = input.read_int();
    let mut a = input.read_int_vec(n);
    let mut happy = 0;
    let mut single = 0;

    for num in a.iter_mut() {
        if *num % 2 == 1 {
            single += 1;
            *num -= 1;
        }
        happy += *num;
    }

    let taken = happy / 2;
    let row_left = r - taken;
    let mut single_happy = single;
    if single_happy > row_left {
        single_happy -= (single_happy - row_left) * 2;
    }

    out.print_line(happy + single_happy);
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
