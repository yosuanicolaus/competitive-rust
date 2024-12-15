//{"name":"D. Sharky Surfing","group":"Codeforces - Codeforces Round 988 (Div. 3)","url":"https://codeforces.com/problemset/problem/2037/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 5 50\n7 14\n30 40\n2 2\n3 1\n3 5\n18 2\n22 32\n4 3 50\n4 6\n15 18\n20 26\n34 38\n1 2\n8 2\n10 2\n1 4 17\n10 14\n1 6\n1 2\n1 2\n16 9\n1 2 10\n5 9\n2 3\n2 2\n","output":"4\n-1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSharkySurfing"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// TODO: WA with cache; TLE without cache

fn surf(
    events: &Vec<(usize, bool, i32)>,
    idx: usize,
    power: i32,
    picked_up: i32,
    cache: &mut HashMap<(usize, i32), (bool, i32)>,
) -> (bool, i32) {
    if idx == events.len() {
        return (true, picked_up);
    } else if let Some(&res) = cache.get(&(idx, power)) {
        return res;
    }

    let (_pos, is_hurdle, data) = events[idx];
    if is_hurdle {
        // data == required power
        if power < data {
            return (false, i32::MAX);
        } else {
            return surf(events, idx + 1, power, picked_up, cache);
        }
    }

    // data == optional additional power
    let res_skip = surf(events, idx + 1, power, picked_up, cache);
    let res_take = surf(events, idx + 1, power + data, picked_up + 1, cache);
    let mut good_res = vec![];
    if res_skip.0 {
        good_res.push(res_skip.1);
    }
    if res_take.0 {
        good_res.push(res_take.1);
    }
    if good_res.is_empty() {
        return (false, i32::MAX);
    }

    let &best_res = good_res.iter().min().unwrap();

    // insert/update cache
    cache.insert((idx, power), (true, best_res));
    (true, best_res)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let _end = input.read_int();
    let mut hurdles: Vec<(usize, i32)> = vec![];
    let mut powers: Vec<(usize, i32)> = vec![];
    let mut cache = HashMap::new();

    for _ in 0..n {
        let h1 = input.read_size();
        let h2 = input.read_size();
        hurdles.push((h1, (h2 - h1 + 2) as i32));
    }

    for _ in 0..m {
        let p1 = input.read_size();
        let p2 = input.read_int();
        powers.push((p1, p2));
    }

    let mut a = 0usize;
    let mut b = 0usize;
    let mut events: Vec<(usize, bool, i32)> = vec![];

    while a < n || b < m {
        if a == n {
            events.push((powers[b].0, false, powers[b].1));
            b += 1;
        } else if b == m {
            events.push((hurdles[a].0, true, hurdles[a].1));
            a += 1;
        } else if hurdles[a].0 < powers[b].0 {
            events.push((hurdles[a].0, true, hurdles[a].1));
            a += 1;
        } else {
            events.push((powers[b].0, false, powers[b].1));
            b += 1;
        }
    }

    let (res_bool, res_data) = surf(&events, 0, 1, 0, &mut cache);
    if res_bool {
        out.println(res_data);
    } else {
        out.println(-1);
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
