use super::ff::{vec_add_ff_p64, vec_mul_ff_p64, MOD, UINT_MAX, ULONG_MAX};
use std::simd::{simd_swizzle, Simd};

const ZEROS_1: Simd<u64, 1> = Simd::from_array([0u64; 1]);
const ONES_1: Simd<u64, 1> = Simd::from_array([1u64; 1]);

#[inline]
fn apply_sbox(state: Simd<u64, 16>) -> Simd<u64, 16> {
  let state2 = vec_mul_ff_p64(state, state);
  let state4 = vec_mul_ff_p64(state2, state2);
  let state6 = vec_mul_ff_p64(state2, state4);

  vec_mul_ff_p64(state, state6)
}

#[inline]
fn apply_constants(state: Simd<u64, 16>, cnst: Simd<u64, 16>) -> Simd<u64, 16> {
  vec_add_ff_p64(state, cnst)
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
fn exp_acc(m: usize, base: Simd<u64, 16>, tail: Simd<u64, 16>) -> Simd<u64, 16> {
  let mut res = base;

  for _ in 0..m {
    res = vec_mul_ff_p64(res, res);
  }

  vec_mul_ff_p64(res, tail)
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
