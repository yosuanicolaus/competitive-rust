//{"name":"C - Various Kagamimochi","group":"AtCoder - HHKB Programming Contest 2025(AtCoder Beginner Contest 388)","url":"https://atcoder.jp/contests/abc388/tasks/abc388_c","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 3 4 4 7 10\n","output":"8\n"},{"input":"3\n387 388 389\n","output":"0\n"},{"input":"32\n1 2 4 5 8 10 12 16 19 25 33 40 50 64 87 101 149 175 202 211 278 314 355 405 412 420 442 481 512 582 600 641\n","output":"388\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVariousKagamimochi"}}}

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

    let mut ans = 0;

    for j in (1..n).rev() {
        // bin search
        let half = a[j] / 2;
        let i = a.partition_point(|&x| x <= half);
        ans += i;
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
    tester::run_main();
}
//END MAIN
