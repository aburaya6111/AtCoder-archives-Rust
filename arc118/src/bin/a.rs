use num::Float;

#[rustfmt::skip]
mod io_pro {
    #[macro_export] macro_rules! input{(sc=$sc:expr,$($r:tt)*)=>{input_inner!{$sc,$($r)*}};($($r:tt)*)=>{let mut sc=io_pro::Scanner::new(std::io::stdin().lock());input_inner!{sc,$($r)*}};}
    #[macro_export] macro_rules! input_inner{($sc:expr)=>{};($sc:expr,)=>{};($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($sc,$t);input_inner!{$sc $($r)*}};}
    #[macro_export] macro_rules! read_value{($sc:expr,($($t:tt),*))=>{($(read_value!($sc,$t)),*)};($sc:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()};($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};($sc:expr,Usize1)=>{read_value!($sc,usize)-1};($sc:expr,$t:ty)=>{$sc.next::<$t>()};}
    pub struct Scanner{s:Box<str>,input:std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,}
    impl Scanner{
        pub fn new<R:std::io::Read>(mut reader:R)->Self{let mut sc=Scanner{s:{let mut s=String::new();reader.read_to_string(&mut s).unwrap();s.into_boxed_str()},input:"".split_ascii_whitespace().peekable(),};let s:&'static str=unsafe{std::mem::transmute(&*sc.s)};sc.input=s.split_ascii_whitespace().peekable();sc}
        #[inline]pub fn next<T:std::str::FromStr>(&mut self)->T where T::Err:std::fmt::Debug,{self.input.next().unwrap().parse::<T>().expect("Parse error")}
    }
}
#[proconio::fastout]
fn main() {
    input!(t: f64, n: i64);
    println!("{}", ((100 * n) as f64 / t) as i64 + n - 1);
}

#[test]
fn aa() {
    let t = 3.0;
    for i in 1..101 {
        println!(
            "{}",
            ((100.0 / t).ceil() * i as f64) as i64
                + (i as f64 / ((100.0 / t).ceil())).floor() as i64
        );
    }
}
#[test]
fn aaa() {
    let t = 3.0;
    let mut set = std::collections::HashMap::new();
    for i in 0..30100 {
        set.insert((((100.0 + t) / 100.0 * i as f64) as i64), i);
    }
    for i in 0..300 {
        if !set.contains_key(&i) {
            println!("{} {}", i, set.get(&i).unwrap_or(&-1));
        }
    }
}

//
#[test]
fn aaaa() {
    let t = 5.0;
    for i in 1..101 {
        println!(
            "{}",
            ((100.0 / t).ceil() * i as f64) as i64
                + (i as f64 / ((100.0 / t).ceil())).floor() as i64
        );
    }
}
