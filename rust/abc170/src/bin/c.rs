use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: isize,
        n: isize,
        ps: [isize; n],
    }

    let mut r: isize = 10isize.pow(4);

    for i in 0..100 {
        let mut j: isize = x - i;

        if !ps.contains(&j) {
            r = j;
            break;
        }

        j = x + i;
        if !ps.contains(&j) {
            r = j;
            break;
        }
    }

    println!("{}", r)
}
