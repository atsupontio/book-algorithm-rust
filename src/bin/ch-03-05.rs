use proconio::input;

fn count_divided_2(mut number: i32) -> i32 {
    let mut count = 0;
    while number % 2 == 0 {
        number /= 2;
        count += 1;
    }
    count
}

fn main() {
    input! {
        N: u8,
        a: [i32; N],
    }

    let mut minimum_index = 10000000;

    for ith_element in a {
        minimum_index = std::cmp::min(count_divided_2(ith_element), minimum_index);
    }
    println!("{}", minimum_index);
}