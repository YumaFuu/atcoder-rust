use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut p: [isize; n]
    }

    p.sort();
    let r = &p[0..m];
    println!("{:?}", r.iter().sum::<isize>());
}
