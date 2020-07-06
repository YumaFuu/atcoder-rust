use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: u32,
        w: u32,
        k: isize,
        cells: [Chars; h],
    }

    let mut wi = 0b0;
    let mut hi = 0b0;
    for _ in 0..=2isize.pow(w) {
        let mut results: Vec<Vec<char>> = vec![vec![]];

        wi += 1;
        let swi = format!("{:0>1$b}", wi, (w as usize));

        for i in 0..=w {
            if swi.chars().nth(*&i as usize).unwrap() == '1' {
                results.push(vec!['.'; (w as usize)]);
            } else {
                let u = &cells[i as usize];
                results.push(u.to_vec());
            }
        }

        for _ in 0..=2isize.pow(h) {
            hi += 1;
            let shi = format!("{:0>1$b}", hi, (h as usize));
        }
    }
}
