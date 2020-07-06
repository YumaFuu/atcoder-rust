use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        xs: [isize; 5]
    }

    let mut r: usize = 0;
    for (i, x) in xs.iter().enumerate() {
        if *x == 0 {
            r = i
        }
    }

    println!("{}", r + 1)
}
