pub fn one(input: String) -> i64 {
    input
        .lines()
        .map(|s| s.parse::<i64>().expect("failed to parse line"))
        .fold(0i64, |sum, e| sum + e / 3 - 2)
}

pub fn two(input: String) -> i64 {
    input
        .lines()
        .map(|s| s.parse::<i64>().expect("failed to parse line"))
        .map(|s: i64| {
            let mut sum = 0i64;
            let mut n = s / 3 - 2;
            loop {
                sum += n;
                n = n / 3 - 2;
                if n < 0 {
                    break;
                }
            }
            sum
        })
        .sum::<i64>()
}
