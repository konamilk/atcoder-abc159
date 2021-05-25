use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        s: Chars
    };

    if s.iter().collect::<String>() != s.iter().rev().collect::<String>() {
        println!("No");
        return
    }

    let left = &s[0..s.len()/2];
    if left.iter().collect::<String>() != left.iter().rev().collect::<String>() {
        println!("No");
        return
    }

    let right = &s[s.len()/2+1..s.len()];
    if right.iter().collect::<String>() != right.iter().rev().collect::<String>() {
        println!("No");
        return
    }

    println!("Yes");
}
