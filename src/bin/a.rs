use num_integer::{gcd, lcm};
#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }
    let lcm = lcm(n, m);
    let gcd = gcd(n, m);
    let mut ans: String = String::new();

    println!("{}", ans.len());
}
