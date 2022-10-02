#![allow(unused_imports)]
use rayon::prelude::*;

fn main() {
    let my_vec = vec![1; 1500];
    let mut result = 0;
    my_vec.iter().for_each(|item| result += item);
    println!("{:?}", result);
}
