//{"name":"F. Easy Demon Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/problemset/problem/2044/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3 3 6\n-2 3 -3\n-2 2 -1\n-1\n1\n-2\n2\n-3\n3\n","output":"NO\nYES\nNO\nNO\nYES\nNO\n"},{"input":"5 5 6\n1 -2 3 0 0\n0 -2 5 0 -3\n4\n-3\n5\n2\n-1\n2\n","output":"YES\nYES\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEasyDemonProblem"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// solved with help from editorial
// lesson: try to formalize to a mathematical equation, and then simplify it!
// Formula; X is B - sum of nums from the _plus_ in grid(y,x)
// X ?= (sa - a[y]) * (sb - b[x])

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let mut c1s = HashSet::new();
    let mut c2s = HashSet::new();
    let mut sa: i64 = 0;
    let mut sb: i64 = 0;

    for _ in 0..n {
        let num = input.read_long();
        sa += num;
        c1s.insert(num);
    }
    for _ in 0..m {
        let num = input.read_long();
        sb += num;
        c2s.insert(num);
    }

    'query: for _ in 0..q {
        let goal = input.read_long();

        // test all divisors of goal
        let sqrt = (goal as f64).abs().sqrt() as i64 + 1;
        for d1 in 1..=sqrt {
            if goal % d1 == 0 {
                let d2 = goal / d1;

                if c1s.contains(&(sa - d1)) && c2s.contains(&(sb - d2))
                    || c1s.contains(&(sa - d2)) && c2s.contains(&(sb - d1))
                    || c1s.contains(&(sa + d1)) && c2s.contains(&(sb + d2))
                    || c1s.contains(&(sa + d2)) && c2s.contains(&(sb + d1))
                {
                    out.println("YES");
                    continue 'query;
                }
            }
        }
        out.println("NO");
    }
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
    tester::run_tests();
}
//END MAIN
