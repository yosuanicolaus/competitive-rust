//{"name":"E - Simultaneous Kagamimochi","group":"AtCoder - HHKB Programming Contest 2025(AtCoder Beginner Contest 388)","url":"https://atcoder.jp/contests/abc388/tasks/abc388_e","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 3 4 4 7 10\n","output":"3\n"},{"input":"3\n387 388 389\n","output":"0\n"},{"input":"24\n307 321 330 339 349 392 422 430 477 481 488 537 541 571 575 602 614 660 669 678 712 723 785 792\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESimultaneousKagamimochi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// WA: 20
// AC: 21 (~50%)
// minimum wrong case: 2 3 4 9 (should create pair of 2-4 & 3-9 (2 pairs))

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = vec![];
    let mut used = vec![false; n];

    for _ in 0..n {
        a.push(input.read_int());
    }

    let mut j: isize = n as isize - 1;
    let mut i: isize = j as isize - 1;
    let mut ans = 0;

    while i >= 0 && j >= 0 {
        let ui = i as usize;
        let uj = j as usize;

        if used[uj] {
            j -= 1;
        } else if i >= j {
            i = j - 1;
        } else if used[ui] {
            i -= 1;
        } else if a[ui] * 2 <= a[uj] {
            ans += 1;
            used[ui] = true;
            used[uj] = true;
            i -= 1;
            j -= 1;
        } else {
            i -= 1;
        }
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
