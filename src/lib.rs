#![feature(portable_simd)]

mod ff;

mod rescue_constants;
pub use rescue_constants::{prepare_ark1, prepare_ark2, prepare_mds};

mod rescue_prime;
pub use rescue_prime::{hash_elements, merge};
