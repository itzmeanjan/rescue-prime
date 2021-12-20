use std::simd::Simd;

pub const ULONG_MAX: u64 = 0xffffffffffffffff;
pub const UINT_MAX: u64 = 0xffffffff;
pub const MOD: u64 = 0xffffffff00000001;
const ZEROS: Simd<u64, 16> = Simd::splat(0u64);
const ONES: Simd<u64, 16> = Simd::splat(1u64);

const ZEROS_: Simd<u64, 4> = Simd::splat(0u64);
const ONES_: Simd<u64, 4> = Simd::splat(1u64);

#[inline]
fn mul_hi(a: Simd<u64, 16>, b: Simd<u64, 16>) -> Simd<u64, 16> {
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
fn mul_hi_internal(a: Simd<u64, 4>, b: Simd<u64, 4>) -> Simd<u64, 4> {
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
fn mul_hi_(a: [Simd<u64, 4>; 3], b: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  [
    mul_hi_internal(a[0], b[0]),
    mul_hi_internal(a[1], b[1]),
    mul_hi_internal(a[2], b[2]),
  ]
}

#[inline]
pub fn vec_mul_ff_p64(a: Simd<u64, 16>, b: Simd<u64, 16>) -> Simd<u64, 16> {
  let ab = a * b;
  let cd = mul_hi(a, b);
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
pub fn vec_mul_ff_p64_internal(a: Simd<u64, 4>, b: Simd<u64, 4>) -> Simd<u64, 4> {
  let ab = a * b;
  let cd = mul_hi_internal(a, b);
  let c = cd & UINT_MAX;
  let d = cd >> 32;

  let tmp0 = ab - d;
  let under0 = ab.lanes_lt(d);
  let tmp1 = under0.select(ONES_, ZEROS_) * UINT_MAX;
  let tmp2 = tmp0 - tmp1;

  let tmp3 = (c << 32) - c;

  let tmp4 = tmp2 + tmp3;
  let over0 = tmp2.lanes_gt(ULONG_MAX - tmp3);
  let tmp5 = over0.select(ONES_, ZEROS_) * UINT_MAX;

  tmp4 + tmp5
}

#[inline]
pub fn vec_mul_ff_p64_(a: [Simd<u64, 4>; 3], b: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  [
    vec_mul_ff_p64_internal(a[0], b[0]),
    vec_mul_ff_p64_internal(a[1], b[1]),
    vec_mul_ff_p64_internal(a[2], b[2]),
  ]
}

#[inline]
pub fn vec_add_ff_p64(a: Simd<u64, 16>, b: Simd<u64, 16>) -> Simd<u64, 16> {
  let b_ok = to_canonical(b);

  let tmp0 = a + b_ok;
  let over0 = a.lanes_gt(ULONG_MAX - b_ok);
  let tmp1 = over0.select(ONES, ZEROS) * UINT_MAX;

  let tmp2 = tmp0 + tmp1;
  let over1 = tmp0.lanes_gt(ULONG_MAX - tmp1);
  let tmp3 = over1.select(ONES, ZEROS) * UINT_MAX;

  tmp2 + tmp3
}

#[inline]
pub fn vec_add_ff_p64_internal(a: Simd<u64, 4>, b: Simd<u64, 4>) -> Simd<u64, 4> {
  // replaced call to `to_canonical` with following
  // modulo division operation
  //
  // suggested here https://github.com/rust-lang/portable-simd/issues/215#issuecomment-997106309
  // for exploration purposes
  let b_ok = b % MOD;

  let tmp0 = a + b_ok;
  let over0 = a.lanes_gt(ULONG_MAX - b_ok);
  let tmp1 = over0.select(ONES_, ZEROS_) * UINT_MAX;

  let tmp2 = tmp0 + tmp1;
  let over1 = tmp0.lanes_gt(ULONG_MAX - tmp1);
  let tmp3 = over1.select(ONES_, ZEROS_) * UINT_MAX;

  tmp2 + tmp3
}

#[inline]
pub fn vec_add_ff_p64_(a: [Simd<u64, 4>; 3], b: [Simd<u64, 4>; 3]) -> [Simd<u64, 4>; 3] {
  [
    vec_add_ff_p64_internal(a[0], b[0]),
    vec_add_ff_p64_internal(a[1], b[1]),
    vec_add_ff_p64_internal(a[2], b[2]),
  ]
}

#[inline]
pub fn to_canonical(a: Simd<u64, 16>) -> Simd<u64, 16> {
  // Following two lines are equivalent to `a % MOD`
  //
  // Just to avoid expensive modulo reduction, I use following
  let over = a.lanes_ge(Simd::from_array([MOD; 16]));
  a - over.select(ONES, ZEROS) * MOD
}

#[cfg(test)]
mod test {
  extern crate rand;
  use super::*;
  use std::convert::TryInto;

  fn random_vector() -> Simd<u64, 16> {
    let mut a = Vec::with_capacity(16);
    for _ in 0..16 {
      a.push(rand::random::<u64>() % MOD);
    }

    let arr: [u64; 16] = a.try_into().unwrap();
    Simd::from_array(arr)
  }

  fn random_vector_() -> Simd<u64, 4> {
    let mut a = Vec::with_capacity(4);
    for _ in 0..4 {
      a.push(rand::random::<u64>() % MOD);
    }

    let arr: [u64; 4] = a.try_into().unwrap();
    Simd::from_array(arr)
  }

  #[test]
  fn test_ff_mul_0() {
    let a = random_vector();
    assert_eq!(
      to_canonical(vec_mul_ff_p64(a, ZEROS)).to_array(),
      [0u64; 16]
    );
  }

  #[test]
  fn test_ff_mul_1() {
    let a = random_vector();
    assert_eq!(
      to_canonical(vec_mul_ff_p64(a, ONES)).to_array(),
      a.to_array()
    );
  }

  #[test]
  fn test_ff_mul_2() {
    let a = Simd::from_array([3u64; 16]);
    let b = Simd::from_array([5u64; 16]);
    assert_eq!(to_canonical(vec_mul_ff_p64(a, b)).to_array(), [15u64; 16]);
  }

  #[test]
  fn test_ff_mul_3() {
    let a = Simd::from_array([MOD - 1; 16]);
    let b = Simd::from_array([2u64; 16]);
    let c = Simd::from_array([4u64; 16]);
    assert_eq!(to_canonical(vec_mul_ff_p64(a, a)).to_array(), [1u64; 16]);
    assert_eq!(to_canonical(vec_mul_ff_p64(a, b)).to_array(), [MOD - 2; 16]);
    assert_eq!(to_canonical(vec_mul_ff_p64(a, c)).to_array(), [MOD - 4; 16]);
  }

  #[test]
  fn test_ff_mul_4() {
    let a = Simd::from_array([(MOD + 1) / 2; 16]);
    let b = Simd::from_array([2u64; 16]);
    assert_eq!(to_canonical(vec_mul_ff_p64(a, b)).to_array(), [1u64; 16]);
  }

  #[test]
  fn test_ff_mul_0_() {
    let a = [random_vector_(), random_vector_(), random_vector_()];
    let b = vec_mul_ff_p64_(a, [ZEROS_; 3]);

    assert_eq!((b[0] % MOD).to_array(), [0u64; 4]);
    assert_eq!((b[1] % MOD).to_array(), [0u64; 4]);
    assert_eq!((b[2] % MOD).to_array(), [0u64; 4]);
  }

  #[test]
  fn test_ff_mul_1_() {
    let a = [random_vector_(), random_vector_(), random_vector_()];
    let b = vec_mul_ff_p64_(a, [ONES_; 3]);

    assert_eq!((b[0] % MOD).to_array(), a[0].to_array());
    assert_eq!((b[1] % MOD).to_array(), a[1].to_array());
    assert_eq!((b[2] % MOD).to_array(), a[2].to_array());
  }

  #[test]
  fn test_ff_mul_2_() {
    let a: Simd<u64, 4> = Simd::splat(3u64);
    let b: Simd<u64, 4> = Simd::splat(5u64);
    assert_eq!((vec_mul_ff_p64_internal(a, b) % MOD).to_array(), [15u64; 4]);
  }

  #[test]
  fn test_ff_mul_3_() {
    let a = [Simd::from_array([MOD - 1; 4]); 3];
    let b = [Simd::from_array([2u64; 4]); 3];
    let c = [Simd::from_array([4u64; 4]); 3];

    let res_0 = vec_mul_ff_p64_(a, a);
    assert_eq!((res_0[0] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_0[1] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_0[2] % MOD).to_array(), [1u64; 4]);

    let res_1 = vec_mul_ff_p64_(a, b);
    assert_eq!((res_1[0] % MOD).to_array(), [MOD - 2; 4]);
    assert_eq!((res_1[1] % MOD).to_array(), [MOD - 2; 4]);
    assert_eq!((res_1[2] % MOD).to_array(), [MOD - 2; 4]);

    let res_2 = vec_mul_ff_p64_(a, c);
    assert_eq!((res_2[0] % MOD).to_array(), [MOD - 4; 4]);
    assert_eq!((res_2[1] % MOD).to_array(), [MOD - 4; 4]);
    assert_eq!((res_2[2] % MOD).to_array(), [MOD - 4; 4]);
  }

  #[test]
  fn test_ff_mul_4_() {
    let a = [Simd::from_array([(MOD + 1) / 2; 4]); 3];
    let b = [Simd::from_array([2u64; 4]); 3];

    let res = vec_mul_ff_p64_(a, b);
    assert_eq!((res[0] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res[1] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res[2] % MOD).to_array(), [1u64; 4]);
  }

  #[test]
  fn test_ff_add_0() {
    let a = random_vector();
    assert_eq!(
      to_canonical(vec_add_ff_p64(a, ZEROS)).to_array(),
      a.to_array()
    );
  }

  #[test]
  fn test_ff_add_1() {
    let a = Simd::from_array([2u64; 16]);
    let b = Simd::from_array([3u64; 16]);
    assert_eq!(to_canonical(vec_add_ff_p64(a, b)).to_array(), [5u64; 16]);
  }

  #[test]
  fn test_ff_add_2() {
    let a = Simd::from_array([MOD - 1; 16]);
    assert_eq!(to_canonical(vec_add_ff_p64(a, ONES)).to_array(), [0u64; 16]);
    assert_eq!(
      to_canonical(vec_add_ff_p64(a, Simd::from_array([2u64; 16]))).to_array(),
      [1u64; 16]
    );
  }

  #[test]
  fn test_ff_add_3() {
    let a = Simd::from_array([MOD - 1; 16]);
    let b = Simd::from_array([0xffffffffu64; 16]);
    assert_eq!(
      to_canonical(vec_add_ff_p64(a, b)).to_array(),
      [0xfffffffeu64; 16]
    );
  }

  #[test]
  fn test_ff_add_0_() {
    let a = [random_vector_(), random_vector_(), random_vector_()];
    let res = vec_add_ff_p64_(a, [ZEROS_; 3]);

    assert_eq!((res[0] % MOD).to_array(), a[0].to_array());
    assert_eq!((res[1] % MOD).to_array(), a[1].to_array());
    assert_eq!((res[2] % MOD).to_array(), a[2].to_array());
  }

  #[test]
  fn test_ff_add_1_() {
    let a = [Simd::from_array([2u64; 4]); 3];
    let b = [Simd::from_array([3u64; 4]); 3];
    let res = vec_add_ff_p64_(a, b);

    assert_eq!((res[0] % MOD).to_array(), [5u64; 4]);
    assert_eq!((res[1] % MOD).to_array(), [5u64; 4]);
    assert_eq!((res[2] % MOD).to_array(), [5u64; 4]);
  }

  #[test]
  fn test_ff_add_2_() {
    let a = [Simd::from_array([MOD - 1; 4]); 3];
    let b = [ONES_; 3];
    let c: [Simd<u64, 4>; 3] = [Simd::splat(2u64); 3];

    let res_0 = vec_add_ff_p64_(a, b);
    assert_eq!((res_0[0] % MOD).to_array(), [0u64; 4]);
    assert_eq!((res_0[1] % MOD).to_array(), [0u64; 4]);
    assert_eq!((res_0[2] % MOD).to_array(), [0u64; 4]);

    let res_1 = vec_add_ff_p64_(a, c);
    assert_eq!((res_1[0] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_1[1] % MOD).to_array(), [1u64; 4]);
    assert_eq!((res_1[2] % MOD).to_array(), [1u64; 4]);
  }

  #[test]
  fn test_ff_add_3_() {
    let a = [Simd::from_array([MOD - 1; 4]); 3];
    let b = [Simd::from_array([0xffffffffu64; 4]); 3];

    let res = vec_add_ff_p64_(a, b);
    assert_eq!((res[0] % MOD).to_array(), [0xfffffffeu64; 4]);
    assert_eq!((res[1] % MOD).to_array(), [0xfffffffeu64; 4]);
    assert_eq!((res[2] % MOD).to_array(), [0xfffffffeu64; 4]);
  }
}
