use std::collections::HashMap;

use crate::util;

fn north(matrix: &mut Vec<Vec<u8>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] != b'O' {
                continue;
            }

            matrix[i][j] = b'.';

            let mut k = i;
            while k > 0 && matrix[k - 1][j] == b'.' {
                k -= 1;
            }

            matrix[k][j] = b'O';
        }
    }
}

fn west(matrix: &mut Vec<Vec<u8>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] != b'O' {
                continue;
            }

            matrix[i][j] = b'.';

            let mut k = j;
            while k > 0 && matrix[i][k - 1] == b'.' {
                k -= 1;
            }

            matrix[i][k] = b'O';
        }
    }
}

fn south(matrix: &mut Vec<Vec<u8>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    for i in (0..m).rev() {
        for j in 0..n {
            if matrix[i][j] != b'O' {
                continue;
            }

            matrix[i][j] = b'.';

            let mut k = i;
            while k + 1 < m && matrix[k + 1][j] == b'.' {
                k += 1;
            }

            matrix[k][j] = b'O';
        }
    }
}

fn east(matrix: &mut Vec<Vec<u8>>) {
    let m = matrix.len();
    let n = matrix[0].len();

    for i in 0..m {
        for j in (0..n).rev() {
            if matrix[i][j] != b'O' {
                continue;
            }

            matrix[i][j] = b'.';

            let mut k = j;
            while k + 1 < m && matrix[i][k + 1] == b'.' {
                k += 1;
            }

            matrix[i][k] = b'O';
        }
    }
}

fn count(matrix: &Vec<Vec<u8>>) -> i64 {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut sum = 0_i64;
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == b'O' {
                sum += (m - i) as i64;
            }
        }
    }

    return sum;
}

fn code(matrix: &mut Vec<Vec<u8>>) -> String {
    let mut sb = String::new();
    for row in matrix {
        for elem in row {
            sb.push(*elem as char);
        }
    }
    return sb;
}

fn solve(matrix: &mut Vec<Vec<u8>>) -> i64 {
    north(matrix);
    count(matrix)
}

fn solve_adv(matrix: &mut Vec<Vec<u8>>) -> i64 {
    let mut counts = Vec::new();
    let mut codes = HashMap::new();

    for i in 0..1000 {
        north(matrix);
        west(matrix);
        south(matrix);
        east(matrix);

        let n = count(matrix);
        let c = code(matrix);

        counts.push(n);

        match codes.get(&c) {
            None => {
                codes.insert(c, i);
            }
            Some(val) => {
                let loop_size = i - *val;
                let loop_remind = (1_000_000_000 - *val - 1) % loop_size;
                return counts[*val + loop_remind];
            }
        }
    }

    panic!()
}

pub fn run(filename: &str, adv: bool) {
    let mut raw: Vec<Vec<u8>> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                raw.push(data.into_bytes())
            }
        }
    }

    println!(
        "{}",
        if adv {
            solve_adv(&mut raw)
        } else {
            solve(&mut raw)
        }
    );
}
