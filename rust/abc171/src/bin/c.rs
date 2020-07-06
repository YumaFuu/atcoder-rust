use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: isize,
    }

    let mut result: Vec<isize> = vec![];

    // n -= 1;
    for i in 0..=11 {
        let i = 11 - i;
        let div = 26isize.pow(i);
        if div > n {
            result.push(100);
            continue;
        }

        // println!("i: {}, n: {}, 26**i: {}", i, n, div);
        // println!("n/26**i: {}", n / div);
        // println!();
        result.push(n / div);
        n = n % div;
    }

    for c in &result {
        if *c == 100 {
            continue;
        }

        let c = std::char::from_u32(*c as u32 + 0x60);
        print!("{}", c.unwrap());
    }
    // println!("{:?}", result);
}
