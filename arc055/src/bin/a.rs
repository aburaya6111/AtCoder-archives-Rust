#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(n: usize);
    print!("1");
    for _ in 1..n {
        print!("0");
    }
    println!("7");
}
