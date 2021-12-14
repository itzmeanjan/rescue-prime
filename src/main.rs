#![feature(portable_simd)]
use std::simd::Simd;

fn main() {
    let a = Simd::from_array([1u64; 16]);
    let b = Simd::from_array([2u64; 16]);

    println!("{:?}", (a + b).to_array());
}
