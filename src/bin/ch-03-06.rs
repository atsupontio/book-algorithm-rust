use proconio::input;

fn problem(k: i32, n: i32) -> u32 {
    let mut ans = 0;
    let m = std::cmp::min(k, n);
    for x in 0..=m {
        for y in 0..=m {
            let z: i32 = n - x - y;
            if z >= 0 && z <= k {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    input! {
        k: i32,
        n: i32,
    }

    let ans = problem(k, n);

    println!("{}", ans);
    
}