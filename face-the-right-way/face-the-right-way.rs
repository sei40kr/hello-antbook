#[allow(unused_imports)]
use std::cmp::Ordering::{Equal, Greater, Less};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};
#[allow(unused_imports)]
use std::iter::FromIterator;

// face-the-right-way.rs --- Face The Right Way
// author: Seong Yong-ju <sei40kr@gmail.com>

// tanakah's input macro
// cf https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),*) => {
        println!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

fn calc(dirs: &Vec<bool>, k: usize) -> Option<u32> {
    let n = dirs.len();
    let mut sum = 0;
    let mut inverted = vec![false; n - k + 1];
    let mut m = 0;

    for (i, &dir) in dirs[0..(n - k + 1)].into_iter().enumerate() {
        if (if dir { 1 } else { 0 } + sum) % 2 == 1 {
            inverted[i] = true;
            sum += 1;

            m += 1;
        }

        if k <= i {
            sum -= if inverted[i - k] { 1 } else { 0 };
        }
    }

    for (i, &dir) in dirs[(n - k + 1)..].into_iter().enumerate() {
        if (if dir { 1 } else { 0 } + sum) % 2 == 1 {
            return None;
        }

        if k <= i {
            sum -= if inverted[i - k] { 1 } else { 0 };
        }
    }

    return Some(m);
}

fn main() {
    input! {
        n: usize,
        dirs: [bool; n]
    };

    let mut ans = None;

    for k in (1..(n + 1)).rev() {
        match calc(&dirs, k) {
            Some(m) => {
                ans = Some((k, m));
                break;
            }
            None => {}
        }
    }

    match ans {
        Some((k, m)) => {
            println!("{} {}", k, m);
        }
        None => {}
    }
}
