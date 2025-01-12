//{"name":"C. The Trail","group":"Codeforces - Codeforces Round 996 (Div. 2)","url":"https://codeforces.com/contest/2055/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 3\nDRRD\n0 2 3\n0 0 0\n3 1 0\n4 5\nDRRRRDD\n0 1 0 2 3\n0 0 0 0 0\n-1 0 -3 -3 0\n0 0 0 -1 0\n2 3\nRRD\n0 0 0\n0 1 0\n5 5\nDDDDRRRR\n0 25 2 9 11\n0 6 13 20 22\n0 17 24 1 8\n0 3 10 12 19\n0 0 0 0 0\n","output":"1 2 3\n2 3 1\n3 1 2\n-6 1 0 2 3\n7 -1 3 2 -11\n-1 0 -3 -3 7\n0 0 0 -1 1\n0 -1 1\n0 1 -1\n18 25 2 9 11\n4 6 13 20 22\n15 17 24 1 8\n21 3 10 12 19\n7 14 16 23 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheTrail"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// couldn't finish in time.

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let trails = input.read_vec_char();
    let mut grid = vec![vec![0; m]; n];
    let mut sh = vec![];
    let mut sv = vec![];

    for y in 0..n {
        let mut th = 0;
        for x in 0..m {
            grid[y][x] = input.read_int();
            th += grid[y][x];
        }
        sh.push(th);
    }
    for x in 0..m {
        let mut tv = 0;
        for y in 0..n {
            tv += grid[y][x];
        }
        sv.push(tv);
    }

    let magic_num = if trails[0] == 'D' { sh[0] } else { sv[0] };
    let mut ty = 0;
    let mut tx = 0;

    dbg!(&magic_num, &trails);

    for (i, &trail) in trails.iter().enumerate() {
        if trail == 'D' {
            // horizontal safe
            ty += 1;
        } else {
            // 'R', vertical safe
            tx += 1;
        }

        if i + 1 < trails.len() {
            let nt = trails[i + 1];
            if nt == 'D' {
                // ver safe
                grid[ty][tx] = magic_num - sv[tx];
                sv[ty] += grid[ty][tx];
            } else {
                // hor safe
                grid[ty][tx] = magic_num - sh[ty];
                sh[tx] += grid[ty][tx];
            }
        } else {
            // last
            if trail == 'D' {
                // hor safe
                grid[ty][tx] = magic_num - sh[ty];
            } else {
                // ver safe
                grid[ty][tx] = magic_num - sv[tx];
            }
        }
    }

    for y in 0..n {
        out.println(&grid[y]);
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
    tester::run_main();
}
//END MAIN
