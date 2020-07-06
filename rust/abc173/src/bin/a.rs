use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
    }

    let n = n % 1000;
    let r = if n == 0 { 0 } else { 1000 - n };
    println!("{}", r);
}
