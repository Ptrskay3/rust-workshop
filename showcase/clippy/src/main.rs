#![allow(dead_code, unused_assignments)]

fn main() {
    let mut m = vec![1; 500];
    let mut s = vec![2; 500];

    m = s;
    s = m;
}
