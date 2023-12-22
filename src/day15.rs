use crate::util;

fn hashsh(s: &str) -> i64 {
    let mut val: i64 = 0;
    for c in s.bytes() {
        val += c as i64;
        val *= 17;
        val %= 256;
    }

    val
}

pub fn run(filename: &str, adv: bool) {
    let mut raw: Vec<String> = Vec::new();
    if let Ok(lines) = util::read_lines(filename) {
        for (_, line) in lines.enumerate() {
            if let Ok(data) = line {
                for part in data.split(',') {
                    raw.push(part.to_owned());
                }
            }
        }
    }

    if !adv {
        let mut sum = 0;
        for r in raw {
            sum += hashsh(&r);
        }

        println!("{}", sum);
        return;
    }

    let mut boxes: Vec<Vec<(String, i32)>> = Vec::new();
    boxes.resize(256, Vec::new());

    for r in raw {
        let del = r.find(|x| x == '-' || x == '=');
        let (label, rest) = r.split_at(del.unwrap());
        let (op, val) = rest.split_at(1);

        let slot = &mut boxes[hashsh(label) as usize];
        if op == "=" {
            let mut set = false;
            for (k, v) in &mut *slot {
                if k == label {
                    set = true;
                    *v = val.parse().unwrap();
                }
            }

            if !set {
                slot.push((label.to_owned(), val.parse().unwrap()))
            }
        } else if op == "-" {
            slot.retain(|x| x.0 != label);
        }
    }

    let mut sum = 0;
    for (box_number, slot) in boxes.iter().enumerate() {
        for (pos, data) in slot.iter().enumerate() {
            sum += (((box_number + 1) * (pos + 1)) as i64) * (data.1 as i64);
        }
    }

    println!("{}", sum);
}
