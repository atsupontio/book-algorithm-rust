use proconio::{input};

fn search753(k: u64, cur: u64, counter: &mut u64, user: u8) {
    if cur > k {
        return;
    }
    if user == 0b111 {
        *counter += 1;
    }

    search753(k, cur * 10 + 7, counter, user | 0b001);
    search753(k, cur * 10 + 5, counter, user | 0b010);
    search753(k, cur * 10 + 3, counter, user | 0b100);

}

fn main() {
    input! {
        k: u64,
    }

    // let k: Vec<u64> = input_s.bytes().map(|c| (c - b'0') as u64).collect();

    let mut counter: u64 = 0;
    search753(k, 0, &mut counter, 0);

    println!("{}", counter);

}
