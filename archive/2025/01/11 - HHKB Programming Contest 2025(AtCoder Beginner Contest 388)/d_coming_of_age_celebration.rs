//{"name":"D - Coming of Age Celebration","group":"AtCoder - HHKB Programming Contest 2025(AtCoder Beginner Contest 388)","url":"https://atcoder.jp/contests/abc388/tasks/abc388_d","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 0 9 3\n","output":"2 0 10 5\n"},{"input":"5\n4 6 7 2 5\n","output":"0 4 7 4 9\n"},{"input":"10\n2 9 1 2 0 4 6 7 1 5\n","output":"0 2 0 0 0 4 7 10 4 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DComingOfAgeCelebration"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut a = vec![];
    let mut nb = vec![0; n];
    let mut ans = vec![];

    for _ in 0..n {
        a.push(input.read_int());
    }

    let mut nobonus = 0;

    for i in 0..n {
        if nb[i] > 0 {
            nobonus += nb[i];
        }
        let bonus = (i as i32) - nobonus;
        let have = a[i] + bonus;

        let left = n - i - 1;
        if have < left as i32 {
            nb[i + have as usize + 1] += 1;
            ans.push(0);
        } else {
            ans.push(have - left as i32);
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
