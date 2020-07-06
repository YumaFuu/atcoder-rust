use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: isize,
        y: isize,
    }

    let mut r = false;
    for i in 0..=x {
        let sum = 4 * (x - i) + 2 * i;

        if sum == y {
            r = true;
            break;
        }
    }

    println!("{}", if r { "Yes" } else { "No" })
}
