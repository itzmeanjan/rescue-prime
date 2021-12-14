use super::ff::{vec_add_ff_p64, vec_mul_ff_p64};
use std::simd::Simd;

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
