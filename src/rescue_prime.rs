use super::ff::{vec_add_ff_p64, vec_add_ff_p64_, vec_mul_ff_p64, MOD};
use std::simd::u64x4;

const NUM_ROUNDS: usize = 7;
const RATE_WIDTH: usize = 8;

#[inline]
fn apply_sbox(state: [u64x4; 3]) -> [u64x4; 3] {
    let state2 = vec_mul_ff_p64(state, state);
    let state4 = vec_mul_ff_p64(state2, state2);
    let state6 = vec_mul_ff_p64(state2, state4);

    vec_mul_ff_p64(state, state6)
}

#[inline]
fn apply_constants(state: [u64x4; 3], cnst: [u64x4; 3]) -> [u64x4; 3] {
    vec_add_ff_p64(state, cnst)
}

#[inline]
fn reduce_add(a: u64, mut b: u64) -> u64 {
    b -= MOD * (b >= MOD) as u64;

    let (res, over) = a.overflowing_add(b);
    res.wrapping_sub(MOD * (over as u64))
}

#[inline]
fn reduce_sum_vec4(a: u64x4) -> u64 {
    let a_ = a.to_array();
    let a0 = reduce_add(a_[0], a_[1]);
    let a1 = reduce_add(a_[2], a_[3]);
    reduce_add(a0, a1)
}

#[inline]
fn reduce_sum(a: [u64x4; 3]) -> u64 {
    let a0 = reduce_sum_vec4(a[0]);
    let a1 = reduce_sum_vec4(a[1]);
    let a2 = reduce_sum_vec4(a[2]);
    reduce_sum_vec4(u64x4::from_array([a0, a1, a2, 0]))
}

#[inline]
fn apply_mds(state: [u64x4; 3], mds: [u64x4; 36]) -> [u64x4; 3] {
    let s0 = reduce_sum(vec_mul_ff_p64(state, mds[0..3].try_into().unwrap()));
    let s1 = reduce_sum(vec_mul_ff_p64(state, mds[3..6].try_into().unwrap()));
    let s2 = reduce_sum(vec_mul_ff_p64(state, mds[6..9].try_into().unwrap()));
    let s3 = reduce_sum(vec_mul_ff_p64(state, mds[9..12].try_into().unwrap()));

    let s4 = reduce_sum(vec_mul_ff_p64(state, mds[12..15].try_into().unwrap()));
    let s5 = reduce_sum(vec_mul_ff_p64(state, mds[15..18].try_into().unwrap()));
    let s6 = reduce_sum(vec_mul_ff_p64(state, mds[18..21].try_into().unwrap()));
    let s7 = reduce_sum(vec_mul_ff_p64(state, mds[21..24].try_into().unwrap()));

    let s8 = reduce_sum(vec_mul_ff_p64(state, mds[24..27].try_into().unwrap()));
    let s9 = reduce_sum(vec_mul_ff_p64(state, mds[27..30].try_into().unwrap()));
    let s10 = reduce_sum(vec_mul_ff_p64(state, mds[30..33].try_into().unwrap()));
    let s11 = reduce_sum(vec_mul_ff_p64(state, mds[33..36].try_into().unwrap()));

    [
        u64x4::from_array([s0, s1, s2, s3]),
        u64x4::from_array([s4, s5, s6, s7]),
        u64x4::from_array([s8, s9, s10, s11]),
    ]
}

#[inline]
fn exp_acc(m: usize, base: [u64x4; 3], tail: [u64x4; 3]) -> [u64x4; 3] {
    let mut res = base;

    for _ in 0..m {
        res = vec_mul_ff_p64(res, res);
    }

    vec_mul_ff_p64(res, tail)
}

#[inline]
fn apply_inv_sbox(state: [u64x4; 3]) -> [u64x4; 3] {
    let t1 = vec_mul_ff_p64(state, state);
    let t2 = vec_mul_ff_p64(t1, t1);

    let t3 = exp_acc(3, t2, t2);
    let t4 = exp_acc(6, t3, t3);
    let t4 = exp_acc(12, t4, t4);

    let t5 = exp_acc(6, t4, t3);
    let t6 = exp_acc(31, t5, t5);

    let a = vec_mul_ff_p64(vec_mul_ff_p64(t6, t6), t5);
    let a = vec_mul_ff_p64(a, a);
    let a = vec_mul_ff_p64(a, a);
    let b = vec_mul_ff_p64(vec_mul_ff_p64(t1, t2), state);

    vec_mul_ff_p64(a, b)
}

#[inline]
fn apply_permutation_round(
    mut state: [u64x4; 3],
    mds: [u64x4; 36],
    ark1: [u64x4; 3],
    ark2: [u64x4; 3],
) -> [u64x4; 3] {
    state = apply_sbox(state);
    state = apply_mds(state, mds);
    state = apply_constants(state, ark1);

    state = apply_inv_sbox(state);
    state = apply_mds(state, mds);
    state = apply_constants(state, ark2);

    state
}

fn apply_rescue_permutation(
    mut state: [u64x4; 3],
    mds: [u64x4; 36],
    ark1: [u64x4; 21],
    ark2: [u64x4; 21],
) -> [u64x4; 3] {
    for i in 0..NUM_ROUNDS {
        state = apply_permutation_round(
            state,
            mds,
            ark1[i * 3..(i + 1) * 3].try_into().unwrap(),
            ark2[i * 3..(i + 1) * 3].try_into().unwrap(),
        );
    }

    state
}

pub fn hash_elements(
    input: &[u64],
    mds: [u64x4; 36],
    ark1: [u64x4; 21],
    ark2: [u64x4; 21],
) -> [u64; 4] {
    let l = input.len();
    let mut state: [u64x4; 3] = [
        u64x4::splat(0u64),
        u64x4::splat(0u64),
        u64x4::from_array([0, 0, 0, l as u64 % MOD]),
    ];

    let mut i = 0;
    for j in (0..l).step_by(4) {
        let input_: u64x4 = if j + 4 <= l {
            u64x4::from_slice(&input[j..j + 4])
        } else {
            let mut a_: Vec<u64> = vec![0, 0, 0, 0];
            let _ = &a_[..(input.len() - j)].copy_from_slice(&input[j..l]);

            u64x4::from_slice(&a_[..])
        };
        state[i >> 2] = vec_add_ff_p64_(state[i >> 2], input_);

        i += 4;
        if i % RATE_WIDTH == 0 {
            state = apply_rescue_permutation(state, mds, ark1, ark2);
            i = 0;
        }
    }

    if i > 0 {
        state = apply_rescue_permutation(state, mds, ark1, ark2);
    }

    state[0].to_array()
}

pub fn merge(input: [u64; 8], mds: [u64x4; 36], ark1: [u64x4; 21], ark2: [u64x4; 21]) -> [u64; 4] {
    let mut state = [
        u64x4::from_slice(&input[0..4]),
        u64x4::from_slice(&input[4..8]),
        u64x4::from_array([0, 0, 0, RATE_WIDTH as u64]),
    ];

    state = apply_rescue_permutation(state, mds, ark1, ark2);
    state[0].to_array()
}

#[cfg(test)]
mod test {
    use super::super::rescue_constants::{prepare_ark1, prepare_ark2, prepare_mds};
    use super::*;

    #[test]
    fn test_apply_sbox() {
        let state: [u64x4; 3] = [
            u64x4::from_array([1 << 10, 1 << 11, 1 << 12, 1 << 13]),
            u64x4::from_array([1 << 20, 1 << 21, 1 << 22, 1 << 23]),
            u64x4::from_array([1 << 60, 1 << 61, 1 << 62, 1 << 63]),
        ];
        let res = apply_sbox(state);

        assert_eq!(
            (res[0] % u64x4::splat(MOD)).to_array(),
            [
                274877906880,
                35184372080640,
                4503599626321920,
                576460752169205760,
            ]
        );
        assert_eq!(
            (res[1] % u64x4::splat(MOD)).to_array(),
            [
                18446726477228539905,
                18444492269600899073,
                18158513693262872577,
                18446744060824649731,
            ]
        );
        assert_eq!(
            (res[2] % u64x4::splat(MOD)).to_array(),
            [
                68719476736,
                8796093022208,
                1125899906842624,
                144115188075855872,
            ]
        );
    }

    #[test]
    fn test_apply_inv_sbox() {
        let state: [u64x4; 3] = [
            u64x4::from_array([1 << 10, 1 << 11, 1 << 12, 1 << 13]),
            u64x4::from_array([1 << 20, 1 << 21, 1 << 22, 1 << 23]),
            u64x4::from_array([1 << 60, 1 << 61, 1 << 62, 1 << 63]),
        ];
        let res = apply_inv_sbox(state);

        assert_eq!(
            (res[0] % u64x4::splat(MOD)).to_array(),
            [
                18446743794536677441,
                536870912,
                4503599626321920,
                18446735273321562113,
            ]
        );
        assert_eq!(
            (res[1] % u64x4::splat(MOD)).to_array(),
            [
                18446726477228539905,
                8,
                288230376151711744,
                18446744069414453249,
            ]
        );
        assert_eq!(
            (res[2] % u64x4::splat(MOD)).to_array(),
            [68719476736, 576460752169205760, 18445618169507741697, 512,]
        );
    }

    #[test]
    fn test_apply_rescue_permutation() {
        let state: [u64x4; 3] = [
            u64x4::from_array([0, 1, 2, 3]),
            u64x4::from_array([4, 5, 6, 7]),
            u64x4::from_array([8, 9, 10, 11]),
        ];
        let mds = prepare_mds();
        let ark1 = prepare_ark1();
        let ark2 = prepare_ark2();
        let res = apply_rescue_permutation(state, mds, ark1, ark2);

        assert_eq!(
            (res[0] % u64x4::splat(MOD)).to_array(),
            [
                10809974140050983728,
                6938491977181280539,
                8834525837561071698,
                6854417192438540779,
            ]
        );
        assert_eq!(
            (res[1] % u64x4::splat(MOD)).to_array(),
            [
                4476630872663101667,
                6292749486700362097,
                18386622366690620454,
                10614098972800193173,
            ]
        );
        assert_eq!(
            (res[2] % u64x4::splat(MOD)).to_array(),
            [
                7543273285584849722,
                9490898458612615694,
                9030271581669113292,
                10101107035874348250,
            ]
        );
    }

    #[test]
    fn test_apply_mds() {
        let state: [u64x4; 3] = [
            u64x4::from_array([0, 1, 2, 3]),
            u64x4::from_array([4, 5, 6, 7]),
            u64x4::from_array([8, 9, 10, 11]),
        ];
        let mds = prepare_mds();
        let res = apply_mds(state, mds);

        assert_eq!(
            (res[0] % u64x4::splat(MOD)).to_array(),
            [
                8268579649362235275,
                2236502997719307940,
                4445585223683938180,
                8490351819144058838,
            ]
        );
        assert_eq!(
            (res[1] % u64x4::splat(MOD)).to_array(),
            [
                17912450758129541069,
                12381447012212465193,
                6444916863184583255,
                5403602327365240081,
            ]
        );
        assert_eq!(
            (res[2] % u64x4::splat(MOD)).to_array(),
            [
                7656905977925454065,
                12880871053868334997,
                13669293285556299269,
                2401034710645280649,
            ]
        );
    }

    #[test]
    fn test_merge() {
        let state: [u64; 8] = [
            1 << 0,
            1 << 1,
            1 << 2,
            1 << 3,
            1 << 4,
            1 << 5,
            1 << 6,
            1 << 7,
        ];
        let mds = prepare_mds();
        let ark1 = prepare_ark1();
        let ark2 = prepare_ark2();

        assert_eq!(
            hash_elements(&state, mds, ark1, ark2),
            merge(state, mds, ark1, ark2)
        );
    }
}
