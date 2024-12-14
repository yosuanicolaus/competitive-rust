//{"name":"E. Negatives and Positives","group":"Codeforces - Codeforces Round 849 (Div. 4)","url":"https://codeforces.com/problemset/problem/1791/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n-1 -1 -1\n5\n1 5 -5 0 2\n3\n1 2 3\n6\n-1 10 9 8 7 6\n2\n-1 -1\n","output":"1\n13\n6\n39\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENegativesAndPositives"}}}

use std::cmp;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_i128_vec(n);
    let mut smallest = i128::MAX;
    let mut neg_count = 0;
    let mut odd_negs = 0;
    let mut ans: i128 = 0;

    for num in a {
        smallest = cmp::min(smallest, num.abs());
        ans += num.abs();
        if num < 0 {
            neg_count += 1;
        } else {
            if neg_count % 2 == 1 {
                odd_negs += 1;
            }
            neg_count = 0;
        }
    }
    if neg_count % 2 == 1 {
        odd_negs += 1;
    }

    if odd_negs % 2 == 1 {
        out.println(ans - 2 * smallest);
    } else {
        out.println(ans);
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
