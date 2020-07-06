use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: isize,
        a: [isize; n],
        q: isize,
        bcs: [[isize; 2]; q],
    }

    let mut h: HashMap<isize, isize> = HashMap::new();
    let mut sum = 0;
    for i in a.iter() {
        *h.entry(*i).or_insert(0) += 1;
        sum += i;
    }

    // println!("{:?}", h);
    // println!("sum: {}", sum);
    for bc in bcs {
        let b = bc[0];
        let c = bc[1];

        let b_cnt = *h.entry(b).or_insert(0);
        *h.entry(c).or_insert(0) += b_cnt;
        *h.entry(b).or_insert(0) = 0;

        let inc = (c - b) * b_cnt;
        sum += inc;
        // println!("inc: {}", inc);
        println!("{}", sum);
    }
}
