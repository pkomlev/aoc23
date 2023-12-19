use crate::util;

fn interpolate(tasks: &Vec<Vec<i64>>) {
    let mut sum = 0;
    for task in tasks {
        let mut tmp = task.clone();
        while !tmp.iter().all(|x| *x == 0) {
            for i in 0..tmp.len() - 1 {
                tmp[i] = tmp[i + 1] - tmp[i];
            }

            sum += tmp[tmp.len() - 1];
            tmp.truncate(tmp.len() - 1);
        }
    }

    println!("{}", sum);
}

fn extrapolate(tasks: &Vec<Vec<i64>>) {
    let mut sum = 0;
    for task in tasks {
        let mut tmp = task.clone();
        let mut beg = Vec::new();
        while !tmp.iter().all(|x| *x == 0) {
            beg.push(tmp[0]);
            for i in 0..tmp.len() - 1 {
                tmp[i] = tmp[i + 1] - tmp[i];
            }

            tmp.truncate(tmp.len() - 1);
        }

        beg.reverse();

        let mut val = 0;
        for x in beg.iter() {
            val = x - val;
        }

        sum += val;
    }

    println!("{}", sum);
}

pub fn run(filename: &str, adv: bool) {
    let mut tasks = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for line in lines {
            if let Ok(data) = line {
                let parsed: Vec<i64> = data.split(' ').map(|x| x.parse().unwrap()).collect();
                tasks.push(parsed);
            }
        }
    }

    if adv {
        extrapolate(&tasks)
    } else {
        interpolate(&tasks)
    }
}
