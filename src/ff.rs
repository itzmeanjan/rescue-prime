use std::simd::Simd;

#[inline]
fn mul_hi(a: Simd<u64, 16>, b: Simd<u64, 16>) -> Simd<u64, 16> {
  let a_lo = a & 0x00000000ffffffff;
  let a_hi = a >> 32;
  let b_lo = b & 0x00000000ffffffff;
  let b_hi = b >> 32;

  let a_x_b_hi = a_hi * b_hi;
  let a_x_b_mid = a_hi * b_lo;
  let b_x_a_mid = b_hi * a_lo;
  let a_x_b_lo = a_lo * b_lo;

  let carry_bit =
    ((a_x_b_mid & 0x00000000ffffffff) + (b_x_a_mid & 0x00000000ffffffff) + (a_x_b_lo >> 32)) >> 32;

  a_x_b_hi + (a_x_b_mid >> 32) + (b_x_a_mid >> 32) + carry_bit
}
