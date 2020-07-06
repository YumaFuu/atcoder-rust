use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        a: [isize; n],
    }

    let mut r = 0;
    for elem in &a {
        r ^= elem;
    }

    for elem in a {
        print!("{} ", elem ^ r);
    }
}
