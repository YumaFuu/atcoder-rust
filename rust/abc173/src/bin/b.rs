use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: isize,
        list: [String; n],
    }

    let mut result = HashMap::new();
    for s in list {
        let s = s.to_string();
        match &*s {
            "AC" => {
                (*result.entry("AC").or_insert(0)) += 1;
            }
            "WA" => {
                (*result.entry("WA").or_insert(0)) += 1;
            }
            "TLE" => {
                (*result.entry("TLE").or_insert(0)) += 1;
            }
            "RE" => {
                (*result.entry("RE").or_insert(0)) += 1;
            }
            _ => {}
        }
    }

    println!("AC x {}", result.entry("AC").or_insert(0));
    println!("WA x {}", result.entry("WA").or_insert(0));
    println!("TLE x {}", result.entry("TLE").or_insert(0));
    println!("RE x {}", result.entry("RE").or_insert(0));
}
