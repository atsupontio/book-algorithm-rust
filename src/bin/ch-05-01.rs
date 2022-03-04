use proconio::{input};

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n],
    }

    // let k: Vec<u64> = input_s.bytes().map(|c| (c - b'0') as u64).collect();

    // dp[i][j]・・・j＋１日目にiを選んだ時のj+1日目までの幸福度の総和
    let mut dp = vec![vec![0isize; 3]; n];
    
    println!("{}", counter);

}
