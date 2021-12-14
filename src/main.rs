#![feature(test)]
#![feature(portable_simd)]

mod ff;
mod rescue_constants;
mod rescue_prime;

fn main() {
    let input: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let mds = rescue_constants::prepare_mds();
    let ark1 = rescue_constants::prepare_ark1();
    let ark2 = rescue_constants::prepare_ark2();

    let hash = rescue_prime::hash_elements(&input, mds, ark1, ark2);
    let merge = rescue_prime::merge(input, mds, ark1, ark2);

    println!("hash  ({:?}) = {:?}", input, hash);
    println!("merge ({:?}) = {:?}", input, merge);
}
