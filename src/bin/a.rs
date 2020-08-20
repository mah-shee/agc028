use num_integer::gcd;
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
    let gcd = gcd(n, m);
    let l = n * m / gcd;

    let n_p = n / gcd;
    let m_p = m / gcd;
    let mut flag = true;

    for i in 0..gcd {
        if s[i * n_p] != t[i * m_p] {
            flag = false;
            break;
        }
    }
    if flag {
        println!("{}", l);
    } else {
        println!("-1");
    }
}
