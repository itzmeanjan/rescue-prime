extern crate simd_rescue_prime;

use simd_rescue_prime::{hash_elements, merge, prepare_ark1, prepare_ark2, prepare_mds};

fn main() {
    let input: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let mds = prepare_mds();
    let ark1 = prepare_ark1();
    let ark2 = prepare_ark2();

    let hash = hash_elements(&input, mds, ark1, ark2);
    let merge = merge(input, mds, ark1, ark2);

    println!("hash  ({:?}) = {:?}", input, hash);
    println!("merge ({:?}) = {:?}", input, merge);
}
