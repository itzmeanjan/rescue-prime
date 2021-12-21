use super::ff::{
  vec_add_ff_p64, vec_add_ff_p64_, vec_mul_ff_p64, vec_mul_ff_p64_, MOD, UINT_MAX, ULONG_MAX,
};
use std::simd::{simd_swizzle, Simd, Which::*};

const ZEROS_1: Simd<u64, 1> = Simd::from_array([0u64; 1]);
const ONES_1: Simd<u64, 1> = Simd::from_array([1u64; 1]);
const NUM_ROUNDS: usize = 7;
const RATE_WIDTH: usize = 8;
const STATE_WIDTH: usize = 12;

#[inline]
fn apply_sbox(state: Simd<u64, 16>) -> Simd<u64, 16> {
  let state2 = vec_mul_ff_p64(state, state);
  let state4 = vec_mul_ff_p64(state2, state2);
  let state6 = vec_mul_ff_p64(state2, state4);

  vec_mul_ff_p64(state, state6)
}

#[inline]
fn apply_sbox_(state: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  let state2 = vec_mul_ff_p64_(state, state);
  let state4 = vec_mul_ff_p64_(state2, state2);
  let state6 = vec_mul_ff_p64_(state2, state4);

  vec_mul_ff_p64_(state, state6)
}

#[inline]
fn apply_constants(state: Simd<u64, 16>, cnst: Simd<u64, 16>) -> Simd<u64, 16> {
  vec_add_ff_p64(state, cnst)
}

#[inline]
fn apply_constants_(state: [Simd<u64, 4>; 3], cnst: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  vec_add_ff_p64_(state, cnst)
}

#[inline]
fn reduce_sum_vec2(a: Simd<u64, 2>) -> u64 {
  let a0 = simd_swizzle!(a, [0]);

  let a1 = simd_swizzle!(a, [1]);
  let over0 = a1.lanes_ge(Simd::from_array([MOD; 1]));
  let a1_ok = a1 - over0.select(ONES_1, ZEROS_1) * MOD;

  let tmp1 = a0 + a1_ok;
  let over1 = a0.lanes_gt(ULONG_MAX - a1_ok);
  let tmp2 = over1.select(ONES_1, ZEROS_1) * UINT_MAX;

  let tmp3 = tmp1 + tmp2;
  let over2 = tmp1.lanes_gt(ULONG_MAX - tmp2);
  let tmp4 = over2.select(ONES_1, ZEROS_1) * UINT_MAX;

  (tmp3 + tmp4).to_array()[0]
}

#[inline]
fn reduce_sum_vec4(a: Simd<u64, 4>) -> u64 {
  let a0 = reduce_sum_vec2(simd_swizzle!(a, [0, 1]));
  let a1 = reduce_sum_vec2(simd_swizzle!(a, [2, 3]));
  reduce_sum_vec2(Simd::from_array([a0, a1]))
}

#[inline]
fn reduce_sum(a: Simd<u64, 16>) -> u64 {
  let a0 = reduce_sum_vec4(simd_swizzle!(a, [0, 1, 2, 3]));
  let a1 = reduce_sum_vec4(simd_swizzle!(a, [4, 5, 6, 7]));
  let a2 = reduce_sum_vec4(simd_swizzle!(a, [8, 9, 10, 11]));
  reduce_sum_vec4(Simd::from_array([a0, a1, a2, 0]))
}

#[inline]
fn reduce_sum_(a: [Simd<u64, 4>; 3]) -> u64 {
  let a0 = reduce_sum_vec4(a[0]);
  let a1 = reduce_sum_vec4(a[1]);
  let a2 = reduce_sum_vec4(a[2]);
  reduce_sum_vec4(Simd::from_array([a0, a1, a2, 0]))
}

#[inline]
fn apply_mds(state: Simd<u64, 16>, mds: [Simd<u64, 16>; 12]) -> Simd<u64, 16> {
  let s0 = reduce_sum(vec_mul_ff_p64(state, mds[0]));
  let s1 = reduce_sum(vec_mul_ff_p64(state, mds[1]));
  let s2 = reduce_sum(vec_mul_ff_p64(state, mds[2]));
  let s3 = reduce_sum(vec_mul_ff_p64(state, mds[3]));
  let s4 = reduce_sum(vec_mul_ff_p64(state, mds[4]));
  let s5 = reduce_sum(vec_mul_ff_p64(state, mds[5]));
  let s6 = reduce_sum(vec_mul_ff_p64(state, mds[6]));
  let s7 = reduce_sum(vec_mul_ff_p64(state, mds[7]));
  let s8 = reduce_sum(vec_mul_ff_p64(state, mds[8]));
  let s9 = reduce_sum(vec_mul_ff_p64(state, mds[9]));
  let s10 = reduce_sum(vec_mul_ff_p64(state, mds[10]));
  let s11 = reduce_sum(vec_mul_ff_p64(state, mds[11]));
  Simd::from_array([s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, 0, 0, 0, 0])
}

#[inline]
fn apply_mds_(state: [Simd<u64, 4>; 3], mds: [Simd<u64, 4>; 36]) -> [Simd<u64, 4>; 3] {
  let s0 = reduce_sum_(vec_mul_ff_p64_(state, mds[0..3].try_into().unwrap()));
  let s1 = reduce_sum_(vec_mul_ff_p64_(state, mds[3..6].try_into().unwrap()));
  let s2 = reduce_sum_(vec_mul_ff_p64_(state, mds[6..9].try_into().unwrap()));
  let s3 = reduce_sum_(vec_mul_ff_p64_(state, mds[9..12].try_into().unwrap()));

  let s4 = reduce_sum_(vec_mul_ff_p64_(state, mds[12..15].try_into().unwrap()));
  let s5 = reduce_sum_(vec_mul_ff_p64_(state, mds[15..18].try_into().unwrap()));
  let s6 = reduce_sum_(vec_mul_ff_p64_(state, mds[18..21].try_into().unwrap()));
  let s7 = reduce_sum_(vec_mul_ff_p64_(state, mds[21..24].try_into().unwrap()));

  let s8 = reduce_sum_(vec_mul_ff_p64_(state, mds[24..27].try_into().unwrap()));
  let s9 = reduce_sum_(vec_mul_ff_p64_(state, mds[27..30].try_into().unwrap()));
  let s10 = reduce_sum_(vec_mul_ff_p64_(state, mds[30..33].try_into().unwrap()));
  let s11 = reduce_sum_(vec_mul_ff_p64_(state, mds[33..36].try_into().unwrap()));

  [
    Simd::from_array([s0, s1, s2, s3]),
    Simd::from_array([s4, s5, s6, s7]),
    Simd::from_array([s8, s9, s10, s11]),
  ]
}

#[inline]
fn exp_acc(m: usize, base: Simd<u64, 16>, tail: Simd<u64, 16>) -> Simd<u64, 16> {
  let mut res = base;

  for _ in 0..m {
    res = vec_mul_ff_p64(res, res);
  }

  vec_mul_ff_p64(res, tail)
}

#[inline]
fn exp_acc_(m: usize, base: [Simd<u64, 4>; 3], tail: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  let mut res = base;

  for _ in 0..m {
    res = vec_mul_ff_p64_(res, res);
  }

  vec_mul_ff_p64_(res, tail)
}

#[inline]
fn apply_inv_sbox(state: Simd<u64, 16>) -> Simd<u64, 16> {
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
fn apply_inv_sbox_(state: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  let t1 = vec_mul_ff_p64_(state, state);
  let t2 = vec_mul_ff_p64_(t1, t1);

  let t3 = exp_acc_(3, t2, t2);
  let t4 = exp_acc_(6, t3, t3);
  let t4 = exp_acc_(12, t4, t4);

  let t5 = exp_acc_(6, t4, t3);
  let t6 = exp_acc_(31, t5, t5);

  let a = vec_mul_ff_p64_(vec_mul_ff_p64_(t6, t6), t5);
  let a = vec_mul_ff_p64_(a, a);
  let a = vec_mul_ff_p64_(a, a);
  let b = vec_mul_ff_p64_(vec_mul_ff_p64_(t1, t2), state);

  vec_mul_ff_p64_(a, b)
}

#[inline]
fn apply_permutation_round(
  mut state: Simd<u64, 16>,
  mds: [Simd<u64, 16>; 12],
  ark1: Simd<u64, 16>,
  ark2: Simd<u64, 16>,
) -> Simd<u64, 16> {
  state = apply_sbox(state);
  state = apply_mds(state, mds);
  state = apply_constants(state, ark1);

  state = apply_inv_sbox(state);
  state = apply_mds(state, mds);
  state = apply_constants(state, ark2);

  state
}

#[inline]
fn apply_permutation_round_(
  mut state: [Simd<u64, 4>; 3],
  mds: [Simd<u64, 4>; 36],
  ark1: [Simd<u64, 4>; 3],
  ark2: [Simd<u64, 4>; 3],
) -> [Simd<u64, 4>; 3] {
  state = apply_sbox_(state);
  state = apply_mds_(state, mds);
  state = apply_constants_(state, ark1);

  state = apply_inv_sbox_(state);
  state = apply_mds_(state, mds);
  state = apply_constants_(state, ark2);

  state
}

fn apply_rescue_permutation(
  mut state: Simd<u64, 16>,
  mds: [Simd<u64, 16>; 12],
  ark1: [Simd<u64, 16>; 7],
  ark2: [Simd<u64, 16>; 7],
) -> Simd<u64, 16> {
  for i in 0..NUM_ROUNDS {
    state = apply_permutation_round(state, mds, ark1[i], ark2[i]);
  }

  state
}

fn apply_rescue_permutation_(
  mut state: [Simd<u64, 4>; 3],
  mds: [Simd<u64, 4>; 36],
  ark1: [Simd<u64, 4>; 21],
  ark2: [Simd<u64, 4>; 21],
) -> [Simd<u64, 4>; 3] {
  for i in 0..NUM_ROUNDS {
    state = apply_permutation_round_(
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
  mds: [Simd<u64, 16>; 12],
  ark1: [Simd<u64, 16>; 7],
  ark2: [Simd<u64, 16>; 7],
) -> [u64; 4] {
  let mut state = {
    let mut arr = [0; 16];
    arr[STATE_WIDTH - 1] = input.len() as u64 % MOD;

    Simd::from_array(arr)
  };

  let mut i = 0;
  for &elm in input.iter() {
    let a = Simd::<u64, 1>::splat(elm);
    let b = match i {
      0 => simd_swizzle!(state, [0]),
      1 => simd_swizzle!(state, [1]),
      2 => simd_swizzle!(state, [2]),
      3 => simd_swizzle!(state, [3]),
      4 => simd_swizzle!(state, [4]),
      5 => simd_swizzle!(state, [5]),
      6 => simd_swizzle!(state, [6]),
      7 => simd_swizzle!(state, [7]),
      _ => simd_swizzle!(state, [0]),
    };
    let c = simd_swizzle!(a, b, [First(0), Second(0)]);
    let d = Simd::<u64, 16>::splat(reduce_sum_vec2(c));

    state = match i {
      0 => {
        simd_swizzle!(
          state,
          d,
          [
            Second(0),
            First(1),
            First(2),
            First(3),
            First(4),
            First(5),
            First(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      1 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            Second(1),
            First(2),
            First(3),
            First(4),
            First(5),
            First(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      2 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            First(1),
            Second(2),
            First(3),
            First(4),
            First(5),
            First(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      3 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            First(1),
            First(2),
            Second(3),
            First(4),
            First(5),
            First(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      4 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            First(1),
            First(2),
            First(3),
            Second(4),
            First(5),
            First(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      5 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            First(1),
            First(2),
            First(3),
            First(4),
            Second(5),
            First(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      6 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            First(1),
            First(2),
            First(3),
            First(4),
            First(5),
            Second(6),
            First(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      7 => {
        simd_swizzle!(
          state,
          d,
          [
            First(0),
            First(1),
            First(2),
            First(3),
            First(4),
            First(5),
            First(6),
            Second(7),
            First(8),
            First(9),
            First(10),
            First(11),
            First(12),
            First(13),
            First(14),
            First(15),
          ]
        )
      }
      _ => state,
    };

    i += 1;
    if i % RATE_WIDTH == 0 {
      state = apply_rescue_permutation(state, mds, ark1, ark2);
      i = 0;
    }
  }

  if i > 0 {
    state = apply_rescue_permutation(state, mds, ark1, ark2);
  }

  simd_swizzle!(state, [0, 1, 2, 3]).to_array()
}

pub fn hash_elements_(
  input: &[u64],
  mds: [Simd<u64, 4>; 36],
  ark1: [Simd<u64, 4>; 21],
  ark2: [Simd<u64, 4>; 21],
) -> [u64; 4] {
  let mut state: [Simd<u64, 4>; 3] = [
    Simd::splat(0),
    Simd::splat(0),
    Simd::from_array([0, 0, 0, input.len() as u64 % MOD]),
  ];

  let mut i = 0;
  for &elm in input.iter() {
    let a = Simd::<u64, 1>::splat(elm);
    let b = match i {
      0 => simd_swizzle!(state[0], [0]),
      1 => simd_swizzle!(state[0], [1]),
      2 => simd_swizzle!(state[0], [2]),
      3 => simd_swizzle!(state[0], [3]),
      4 => simd_swizzle!(state[1], [0]),
      5 => simd_swizzle!(state[1], [1]),
      6 => simd_swizzle!(state[1], [2]),
      7 => simd_swizzle!(state[1], [3]),
      _ => simd_swizzle!(state[2], [0]), // control flow should never arrive here !
    };
    let c = simd_swizzle!(a, b, [First(0), Second(0)]);
    let d = Simd::<u64, 4>::splat(reduce_sum_vec2(c));

    match i {
      0 => state[0] = simd_swizzle!(state[0], d, [Second(0), First(1), First(2), First(3),]),
      1 => state[0] = simd_swizzle!(state[0], d, [First(0), Second(1), First(2), First(3),]),
      2 => state[0] = simd_swizzle!(state[0], d, [First(0), First(1), Second(2), First(3),]),
      3 => state[0] = simd_swizzle!(state[0], d, [First(0), First(1), First(2), Second(3),]),
      4 => state[1] = simd_swizzle!(state[1], d, [Second(0), First(1), First(2), First(3),]),
      5 => state[1] = simd_swizzle!(state[1], d, [First(0), Second(1), First(2), First(3),]),
      6 => state[1] = simd_swizzle!(state[1], d, [First(0), First(1), Second(2), First(3),]),
      7 => state[1] = simd_swizzle!(state[1], d, [First(0), First(1), First(2), Second(3),]),
      _ => {} // control flow should never arrive here !
    };

    i += 1;
    if i % RATE_WIDTH == 0 {
      state = apply_rescue_permutation_(state, mds, ark1, ark2);
      i = 0;
    }
  }

  if i > 0 {
    state = apply_rescue_permutation_(state, mds, ark1, ark2);
  }

  state[0].to_array()
}

pub fn merge(
  input: [u64; 8],
  mds: [Simd<u64, 16>; 12],
  ark1: [Simd<u64, 16>; 7],
  ark2: [Simd<u64, 16>; 7],
) -> [u64; 4] {
  let mut state = {
    let mut arr = [0; 16];
    arr[..RATE_WIDTH].copy_from_slice(&input[..]);
    arr[STATE_WIDTH - 1] = RATE_WIDTH as u64;

    Simd::from_array(arr)
  };

  state = apply_rescue_permutation(state, mds, ark1, ark2);
  simd_swizzle!(state, [0, 1, 2, 3]).to_array()
}

pub fn merge_(
  input: [u64; 8],
  mds: [Simd<u64, 4>; 36],
  ark1: [Simd<u64, 4>; 21],
  ark2: [Simd<u64, 4>; 21],
) -> [u64; 4] {
  let mut state = [
    Simd::from_slice(&input[0..4]),
    Simd::from_slice(&input[4..8]),
    Simd::from_array([0, 0, 0, RATE_WIDTH as u64]),
  ];

  state = apply_rescue_permutation_(state, mds, ark1, ark2);
  state[0].to_array()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_apply_sbox() {
    let state: [Simd<u64, 4>; 3] = [
      Simd::from_array([1 << 10, 1 << 11, 1 << 12, 1 << 13]),
      Simd::from_array([1 << 20, 1 << 21, 1 << 22, 1 << 23]),
      Simd::from_array([1 << 60, 1 << 61, 1 << 62, 1 << 63]),
    ];
    let res = apply_sbox_(state);

    assert_eq!(
      (res[0] % MOD).to_array(),
      [
        274877906880,
        35184372080640,
        4503599626321920,
        576460752169205760,
      ]
    );
    assert_eq!(
      (res[1] % MOD).to_array(),
      [
        18446726477228539905,
        18444492269600899073,
        18158513693262872577,
        18446744060824649731,
      ]
    );
    assert_eq!(
      (res[2] % MOD).to_array(),
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
    let state: [Simd<u64, 4>; 3] = [
      Simd::from_array([1 << 10, 1 << 11, 1 << 12, 1 << 13]),
      Simd::from_array([1 << 20, 1 << 21, 1 << 22, 1 << 23]),
      Simd::from_array([1 << 60, 1 << 61, 1 << 62, 1 << 63]),
    ];
    let res = apply_inv_sbox_(state);

    assert_eq!(
      (res[0] % MOD).to_array(),
      [
        18446743794536677441,
        536870912,
        4503599626321920,
        18446735273321562113,
      ]
    );
    assert_eq!(
      (res[1] % MOD).to_array(),
      [
        18446726477228539905,
        8,
        288230376151711744,
        18446744069414453249,
      ]
    );
    assert_eq!(
      (res[2] % MOD).to_array(),
      [68719476736, 576460752169205760, 18445618169507741697, 512,]
    );
  }

  #[test]
  fn test_apply_rescue_permutation() {
    let state: [Simd<u64, 4>; 3] = [
      Simd::from_array([0, 1, 2, 3]),
      Simd::from_array([4, 5, 6, 7]),
      Simd::from_array([8, 9, 10, 11]),
    ];
    let mds = crate::rescue_constants::prepare_mds_();
    let ark1 = crate::rescue_constants::prepare_ark1_();
    let ark2 = crate::rescue_constants::prepare_ark2_();
    let res = apply_rescue_permutation_(state, mds, ark1, ark2);

    assert_eq!(
      (res[0] % MOD).to_array(),
      [
        10809974140050983728,
        6938491977181280539,
        8834525837561071698,
        6854417192438540779,
      ]
    );
    assert_eq!(
      (res[1] % MOD).to_array(),
      [
        4476630872663101667,
        6292749486700362097,
        18386622366690620454,
        10614098972800193173,
      ]
    );
    assert_eq!(
      (res[2] % MOD).to_array(),
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
    let state: [Simd<u64, 4>; 3] = [
      Simd::from_array([0, 1, 2, 3]),
      Simd::from_array([4, 5, 6, 7]),
      Simd::from_array([8, 9, 10, 11]),
    ];
    let mds = crate::rescue_constants::prepare_mds_();
    let res = apply_mds_(state, mds);

    assert_eq!(
      (res[0] % MOD).to_array(),
      [
        8268579649362235275,
        2236502997719307940,
        4445585223683938180,
        8490351819144058838,
      ]
    );
    assert_eq!(
      (res[1] % MOD).to_array(),
      [
        17912450758129541069,
        12381447012212465193,
        6444916863184583255,
        5403602327365240081,
      ]
    );
    assert_eq!(
      (res[2] % MOD).to_array(),
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
    let mds = crate::rescue_constants::prepare_mds_();
    let ark1 = crate::rescue_constants::prepare_ark1_();
    let ark2 = crate::rescue_constants::prepare_ark2_();

    assert_eq!(
      hash_elements_(&state, mds, ark1, ark2),
      merge_(state, mds, ark1, ark2)
    );
  }
}
