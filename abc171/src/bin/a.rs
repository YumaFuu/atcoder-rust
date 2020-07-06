use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: char
    }

    let r = if a.is_lowercase() { "a" } else { "A" };
    println!("{}", r)
}
