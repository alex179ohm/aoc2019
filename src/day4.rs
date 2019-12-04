
pub fn one() -> usize {
    let mut res: Vec<i32> = Vec::new();
    for ref mut i in 246540..=787419 {
        let d = get_digits(*i);
        if (d[0] == d[1] || d[1] == d[2] || d[2] == d[3] || d[3] == d[4] || d[4] == d[5]) &&
            d[0] <= d[1] && d[1] <= d[2] && d[2] <= d[3] && d[3] <= d[4] && d[4] <= d[5] {
            res.push(*i);
        }
    }
    res.len()
}

pub fn two() -> usize {
    let mut res: Vec<i32> = Vec::new();
    for ref mut i in 246540..=787419 {
        let d = get_digits(*i);
        if d[0] <= d[1] && d[1] <= d[2] && d[2] <= d[3] && d[3] <= d[4] && d[4] <= d[5] && double(&d) {
            res.push(*i);
        }
    }
    res.len()
}

fn double(d: &[i32]) -> bool {
    (d[0] == d[1] && d[1] != d[2])
    || (d[1] == d[2] && d[1] != d[0] && d[2] != d[3])
    || (d[2] == d[3] && d[2] != d[1] && d[3] != d[4])
    || (d[3] == d[4] && d[3] != d[2] && d[4] != d[5])
    || (d[4] == d[5] && d[4] != d[3])
}

fn get_digits(mut n: i32) -> Vec<i32> {
    let mut digits: Vec<i32> = Vec::new();
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}
