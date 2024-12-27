//{"name":"E. Best Price","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 0\n2 1\n3 4\n1 1\n2\n5\n3 3\n1 5 2\n3 6 4\n4 3\n2 3 2 8\n3 7 3 9\n3 1\n2 9 5\n12 14 9\n","output":"2\n5\n9\n14\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBestPrice"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type PreCalc = ();

// with editorial help
// why: misunderstood; above b, there will be no negative review
// (thought non-buyer can still review)

fn update_counts(bad: &mut i32, buyer: &mut i64, review: bool) {
    if review {
        *bad += 1;
    } else {
        *bad -= 1;
        *buyer -= 1;
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_int();
    let mut pq = BinaryHeap::new();

    for _ in 0..n {
        pq.push((Reverse(input.read_int()), true));
    }
    for _ in 0..n {
        pq.push((Reverse(input.read_int()), false));
    }

    let mut ans: i64 = 0;
    let mut buyer = n as i64;
    let mut bad = 0;

    while !pq.is_empty() {
        let (Reverse(price), review) = pq.pop().unwrap();
        if bad <= k {
            ans = cmp::max(ans, price as i64 * buyer);
        }
        update_counts(&mut bad, &mut buyer, review);

        while !pq.is_empty() && pq.peek().unwrap().0 == Reverse(price) {
            let (_, review) = pq.pop().unwrap();
            update_counts(&mut bad, &mut buyer, review);
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
