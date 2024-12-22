//{"name":"D. Counting Pairs","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4 8 10\n4 6 3 6\n6 22 27\n4 9 6 3 4 5\n3 8 10\n3 2 1\n3 1 1\n2 3 4\n3 3 6\n3 2 1\n4 4 12\n3 3 2 1\n6 8 8\n1 1 2 2 2 3\n","output":"4\n7\n0\n0\n1\n5\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountingPairs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp;

type PreCalc = ();

fn get_bound_len(a: &Vec<i64>, lb: i64, ub: i64, curr_idx: usize) -> i64 {
    // binary search
    if ub < lb {
        return 0;
    }
    let mut low = a.partition_point(|&x| x < lb);
    let mut hi = a.partition_point(|&x| x <= ub);

    low = cmp::max(low, curr_idx + 1);
    hi = cmp::max(hi, curr_idx + 1);
    (hi - low) as i64
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_long();
    let y = input.read_long();
    let mut a = vec![];
    let mut hb = 0;

    for _ in 0..n {
        let num = input.read_long();
        hb = cmp::max(hb, num);
        a.push(num);
    }

    let sa: i64 = a.iter().sum();
    a.sort();

    let mut ans = 0;

    for i in 0..n {
        let mut lb = sa - y - a[i];
        let mut ub = sa - x - a[i];

        if lb < 1 {
            lb = 1;
        }
        if ub > hb {
            ub = hb;
        }

        let blen = get_bound_len(&a, lb, ub, i);
        ans += blen;
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
