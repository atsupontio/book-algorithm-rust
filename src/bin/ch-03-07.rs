use proconio::{input};

// bit full search
fn main() {
    input! {
        input_s: String,
    }

    // バイト表現で数字は連続だから引き算すれば整数に変換できるというあれ。
    // 逆順にしておくと数え上げがやりやすい。
    let digits: Vec<u64> = input_s.bytes().map(|c| (c - b'0') as u64).rev().collect();
    let mut sum = 0;
    let n = input_s.len();
    for bit in 0..(1 << n - 1) {
        sum += digits[0];
        let mut count = 0;
        for index in 1..n {
            if bit & (1 << index - 1) != 0 {
                count = 0;
            } else {
                count += 1;
            }

            sum += digits[index] * 10u64.pow(count);
        }
    }
    println!("{}", sum);
}
