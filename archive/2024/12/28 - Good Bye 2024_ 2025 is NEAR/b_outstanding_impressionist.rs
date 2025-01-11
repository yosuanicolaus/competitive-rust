//{"name":"B. Outstanding Impressionist","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 1\n1 1\n4\n1 3\n1 3\n1 3\n1 3\n6\n3 6\n2 2\n1 2\n1 1\n3 4\n2 2\n7\n3 4\n4 4\n4 4\n1 3\n2 5\n1 4\n2 2\n3\n4 5\n4 4\n5 5\n","output":"00\n1111\n100110\n1001111\n011\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOutstandingImpressionist"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut fixed = HashMap::new();
    let mut a = vec![];

    for _ in 0..n {
        let l = input.read_int();
        let r = input.read_int();
        a.push((l, r));

        if l == r {
            *fixed.entry(l).or_insert(0) += 1;
        }
    }

    let mut sf: Vec<i32> = fixed.clone().into_keys().collect();
    sf.sort();
    let mut ranges = HashMap::new();
    for i in 0..sf.len() {
        if i == 0 {
            ranges.insert(sf[i], sf[i]);
        } else {
            if sf[i - 1] + 1 == sf[i] {
                let prev_start = *ranges.get(&sf[i - 1]).unwrap();
                ranges.insert(sf[i], prev_start);
            } else {
                ranges.insert(sf[i], sf[i]);
            }
        }
    }

    let mut lengths = HashMap::new();
    for i in 0..sf.len() {
        if i == 0 {
            lengths.insert(sf[i], 1);
        } else {
            let k = sf[i];
            if *ranges.get(&k).unwrap() == k {
                lengths.insert(k, 1);
            } else {
                let start = *ranges.get(&k).unwrap();
                lengths.entry(start).and_modify(|c| *c += 1);
            }
        }
    }

    let mut ans = vec![];

    'i: for i in 0..n {
        let (l, r) = a[i];

        if l == r {
            if *fixed.get(&l).unwrap() == 1 {
                ans.push('1');
            } else {
                ans.push('0');
            }
        } else {
            if fixed.get(&l).is_none() || fixed.get(&r).is_none() {
                ans.push('1');
                continue 'i;
            }

            // search through ranges & lengths
            let lfs = *ranges.get(&l).unwrap();
            let lfl = *lengths.get(&lfs).unwrap();
            if lfs + lfl <= r {
                ans.push('1');
            } else {
                ans.push('0');
            }
        }
    }

    out.println(ans.iter().collect::<String>());
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
    tester::run_main();
}
//END MAIN
