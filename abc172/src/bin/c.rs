use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        mut a: [isize; n],
        mut b: [isize; m],
    }

    let mut a_acc = vec![0; n + 1];
    let mut b_acc = vec![0; m + 1];

    for i in 0..n {
        a_acc[i + 1] = a[i] + a_acc[i];
    }

    for i in 0..m {
        b_acc[i + 1] = b[i] + b_acc[i];
    }

    let mut ans = 0;
    for i in 0..=n {
        if a_acc[i] > k {
            break;
        }

        let a_cnt = i;
        let mut b_cnt = 0;

        for j in 0..=m {
            let time = a_acc[i] + b_acc[m - j];
            if time > k {
                continue;
            }

            b_cnt = m - j;
            break;
        }

        let sum = a_cnt + b_cnt;
        ans = if sum > ans { sum } else { ans };
    }

    println!("{}", ans)
}
