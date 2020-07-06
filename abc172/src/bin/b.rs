use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    let mut cnt = 0;
    let v: Vec<char> = t.chars().collect();

    for (i, c) in s.chars().enumerate() {
        if c != v[i] {
            cnt += 1;
        }
    }

    println!("{}", cnt)
}
