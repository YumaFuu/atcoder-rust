use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: isize,
        a: [isize; n],
    }

    let mut result = vec![true; 1000001];
    let mut used: Vec<isize> = vec![];
    for i in &a {
        if used.iter().any(|&e| e == *i as isize) {
            result[*i as usize] = false;
            continue;
        }
        used.push(*i);
        for j in 2..10001 {
            let num = &(i * j);
            if num > &1000001isize {
                break;
            }

            result[(*num as usize)] = false;
        }
    }

    let mut sum = 0;
    for i in a {
        if result[i as usize] {
            sum += 1;
        }
    }
    println!("{}", sum);
}
