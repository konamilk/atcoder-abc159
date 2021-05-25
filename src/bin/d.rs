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
        n: usize,
        a: [usize; n]
    };

    let mut bucket = vec![0i64;n+1];
    for i in 0..n{
        bucket[a[i]] += 1;
    }

    let mut no_ban = 0i64;
    for (i,&b) in bucket.iter().enumerate(){
        no_ban += b * (b-1) /2
    }

    for i in 0..n{
        let b = bucket[a[i]];
        let query = - b * (b-1) / 2 + (b-1) * (b-2) / 2;

        println!("{}", no_ban+query)
    }
}
