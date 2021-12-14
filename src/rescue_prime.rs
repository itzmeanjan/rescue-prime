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
