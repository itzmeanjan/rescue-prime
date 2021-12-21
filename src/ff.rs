use std::simd::Simd;

pub const ULONG_MAX: u64 = 0xffffffffffffffff;
pub const UINT_MAX: u64 = 0xffffffff;
pub const MOD: u64 = 0xffffffff00000001;

const ZEROS: Simd<u64, 4> = Simd::splat(0u64);
const ONES: Simd<u64, 4> = Simd::splat(1u64);

#[inline]
fn mul_hi_(a: Simd<u64, 4>, b: Simd<u64, 4>) -> Simd<u64, 4> {
  let a_lo = a & UINT_MAX;
  let a_hi = a >> 32;
  let b_lo = b & UINT_MAX;
  let b_hi = b >> 32;

  let a_x_b_hi = a_hi * b_hi;
  let a_x_b_mid = a_hi * b_lo;
  let b_x_a_mid = b_hi * a_lo;
  let a_x_b_lo = a_lo * b_lo;

  let carry_bit = ((a_x_b_mid & UINT_MAX) + (b_x_a_mid & UINT_MAX) + (a_x_b_lo >> 32)) >> 32;

  a_x_b_hi + (a_x_b_mid >> 32) + (b_x_a_mid >> 32) + carry_bit
}

#[inline]
fn vec_mul_ff_p64_(a: Simd<u64, 4>, b: Simd<u64, 4>) -> Simd<u64, 4> {
  let ab = a * b;
  let cd = mul_hi_(a, b);
  let c = cd & UINT_MAX;
  let d = cd >> 32;

  let tmp0 = ab - d;
  let under0 = ab.lanes_lt(d);
  let tmp1 = under0.select(ONES, ZEROS) * UINT_MAX;
  let tmp2 = tmp0 - tmp1;

  let tmp3 = (c << 32) - c;

  let tmp4 = tmp2 + tmp3;
  let over0 = tmp2.lanes_gt(ULONG_MAX - tmp3);
  let tmp5 = over0.select(ONES, ZEROS) * UINT_MAX;

  tmp4 + tmp5
}

#[inline]
pub fn vec_mul_ff_p64(a: [Simd<u64, 4>; 3], b: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  [
    vec_mul_ff_p64_(a[0], b[0]),
    vec_mul_ff_p64_(a[1], b[1]),
    vec_mul_ff_p64_(a[2], b[2]),
  ]
}

#[inline]
pub fn vec_add_ff_p64_(a: Simd<u64, 4>, b: Simd<u64, 4>) -> Simd<u64, 4> {
  // replaced call to `to_canonical` ( more https://github.com/itzmeanjan/simd-rescue-prime/blob/46d9e8b/src/ff.rs#L62-L69 )
  // with following modulo division operation
  //
  // suggested here https://github.com/rust-lang/portable-simd/issues/215#issuecomment-997106309
  // for exploration purposes
  let b_ok = b % MOD;

  let tmp0 = a + b_ok;
  let over0 = a.lanes_gt(ULONG_MAX - b_ok);
  let tmp1 = over0.select(ONES, ZEROS) * UINT_MAX;

  let tmp2 = tmp0 + tmp1;
  let over1 = tmp0.lanes_gt(ULONG_MAX - tmp1);
  let tmp3 = over1.select(ONES, ZEROS) * UINT_MAX;

  tmp2 + tmp3
}

#[inline]
pub fn vec_add_ff_p64(a: [Simd<u64, 4>; 3], b: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  [
    vec_add_ff_p64_(a[0], b[0]),
    vec_add_ff_p64_(a[1], b[1]),
    vec_add_ff_p64_(a[2], b[2]),
  ]
}

#[cfg(test)]
mod test {
  extern crate rand;
  use super::*;
  use std::convert::TryInto;

  fn random_vector() -> Simd<u64, 4> {
    let mut a = Vec::with_capacity(4);
    for _ in 0..4 {
      a.push(rand::random::<u64>() % MOD);
    }

    let arr: [u64; 4] = a.try_into().unwrap();
    Simd::from_array(arr)
  }

  #[test]
  fn test_ff_mul_1_() {
    let a = [random_vector(), random_vector(), random_vector()];
    let b = vec_mul_ff_p64(a, [ONES; 3]);

    assert_eq!((b[0] % MOD).to_array(), a[0].to_array());
    assert_eq!((b[1] % MOD).to_array(), a[1].to_array());
    assert_eq!((b[2] % MOD).to_array(), a[2].to_array());
  }

  #[test]
  fn test_ff_mul_2_() {
    let a: Simd<u64, 4> = Simd::splat(3u64);
    let b: Simd<u64, 4> = Simd::splat(5u64);
    assert_eq!((vec_mul_ff_p64_(a, b) % MOD).to_array(), [15u64; 4]);
  }

  #[test]
  fn test_ff_mul_3_() {
    let a = [Simd::from_array([MOD - 1; 4]); 3];
    let b = [Simd::from_array([2u64; 4]); 3];
    let c = [Simd::from_array([4u64; 4]); 3];

    let res_0 = vec_mul_ff_p64(a, a);
    assert_eq!((res_0[0] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_0[1] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_0[2] % MOD).to_array(), [1u64; 4]);

    let res_1 = vec_mul_ff_p64(a, b);
    assert_eq!((res_1[0] % MOD).to_array(), [MOD - 2; 4]);
    assert_eq!((res_1[1] % MOD).to_array(), [MOD - 2; 4]);
    assert_eq!((res_1[2] % MOD).to_array(), [MOD - 2; 4]);

    let res_2 = vec_mul_ff_p64(a, c);
    assert_eq!((res_2[0] % MOD).to_array(), [MOD - 4; 4]);
    assert_eq!((res_2[1] % MOD).to_array(), [MOD - 4; 4]);
    assert_eq!((res_2[2] % MOD).to_array(), [MOD - 4; 4]);
  }

  #[test]
  fn test_ff_mul_4_() {
    let a = [Simd::from_array([(MOD + 1) / 2; 4]); 3];
    let b = [Simd::from_array([2u64; 4]); 3];

    let res = vec_mul_ff_p64(a, b);
    assert_eq!((res[0] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res[1] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res[2] % MOD).to_array(), [1u64; 4]);
  }

  #[test]
  fn test_ff_add_0_() {
    let a = [random_vector(), random_vector(), random_vector()];
    let res = vec_add_ff_p64(a, [ZEROS; 3]);

    assert_eq!((res[0] % MOD).to_array(), a[0].to_array());
    assert_eq!((res[1] % MOD).to_array(), a[1].to_array());
    assert_eq!((res[2] % MOD).to_array(), a[2].to_array());
  }

  #[test]
  fn test_ff_add_1_() {
    let a = [Simd::from_array([2u64; 4]); 3];
    let b = [Simd::from_array([3u64; 4]); 3];
    let res = vec_add_ff_p64(a, b);

    assert_eq!((res[0] % MOD).to_array(), [5u64; 4]);
    assert_eq!((res[1] % MOD).to_array(), [5u64; 4]);
    assert_eq!((res[2] % MOD).to_array(), [5u64; 4]);
  }

  #[test]
  fn test_ff_add_2_() {
    let a = [Simd::from_array([MOD - 1; 4]); 3];
    let b = [ONES; 3];
    let c: [Simd<u64, 4>; 3] = [Simd::splat(2u64); 3];

    let res_0 = vec_add_ff_p64(a, b);
    assert_eq!((res_0[0] % MOD).to_array(), [0u64; 4]);
    assert_eq!((res_0[1] % MOD).to_array(), [0u64; 4]);
    assert_eq!((res_0[2] % MOD).to_array(), [0u64; 4]);

    let res_1 = vec_add_ff_p64(a, c);
    assert_eq!((res_1[0] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_1[1] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_1[2] % MOD).to_array(), [1u64; 4]);
  }

  #[test]
  fn test_ff_add_3_() {
    let a = [Simd::from_array([MOD - 1; 4]); 3];
    let b = [Simd::from_array([0xffffffffu64; 4]); 3];

    let res = vec_add_ff_p64(a, b);
    assert_eq!((res[0] % MOD).to_array(), [0xfffffffeu64; 4]);
    assert_eq!((res[1] % MOD).to_array(), [0xfffffffeu64; 4]);
    assert_eq!((res[2] % MOD).to_array(), [0xfffffffeu64; 4]);
  }
}
