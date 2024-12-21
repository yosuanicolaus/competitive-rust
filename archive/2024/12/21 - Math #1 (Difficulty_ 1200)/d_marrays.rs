//{"name":"D. M-arrays","group":"Codeforces - Math #1 (Difficulty: 1200)","url":"https://codeforces.com/group/JESCgZZ8qn/contest/333990/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n6 4\n2 2 8 6 9 4\n10 8\n1 1 1 5 2 4 4 8 6 7\n1 1\n666\n2 2\n2 4\n","output":"3\n6\n1\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::HashMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: i32 = input.read();
    let mut mods_count: HashMap<i32, i32> = HashMap::new();

    for _ in 0..n {
        let num: i32 = input.read();
        *mods_count.entry(num % m).or_insert(0) += 1;
    }

    let mut ans = 0;
    let mut endpair = m / 2;
    if m % 2 == 0 {
        endpair -= 1;
        if mods_count.contains_key(&(m / 2)) {
            ans += 1;
        }
    }
    if mods_count.contains_key(&0) {
        ans += 1;
    }

    for a in 1..=endpair {
        let b = m - a;
        let ca = *mods_count.get(&a).unwrap_or(&0);
        let cb = *mods_count.get(&b).unwrap_or(&0);

        if ca == 0 && cb == 0 {
            continue;
        }

        ans += 1;
        let diff = (ca - cb).abs();
        if diff >= 2 {
            ans += diff - 1;
        }
    }

    out.println(ans);
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
