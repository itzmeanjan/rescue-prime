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

#[inline]
fn vec_mul_ff_p64(a: Simd<u64, 16>, b: Simd<u64, 16>) -> Simd<u64, 16> {
  const zeros: Simd<u64, 16> = Simd::from_array([0u64; 16]);
  const ones: Simd<u64, 16> = Simd::from_array([1u64; 16]);
  const ULONG_MAX: u64 = 0xffffffffffffffff;

  let ab = a * b;
  let cd = mul_hi(a, b);
  let c = cd & 0x00000000ffffffff;
  let d = cd >> 32;

  let tmp0 = ab - d;
  let under0 = ab.lanes_lt(d);
  let tmp1 = under0.select(ones, zeros) * 0xffffffff;
  let tmp2 = tmp0 - tmp1;

  let tmp3 = (c << 32) - c;

  let tmp4 = tmp2 + tmp3;
  let over0 = tmp2.lanes_lt(ULONG_MAX - tmp3);
  let tmp5 = over0.select(ones, zeros) * 0xffffffff;

  tmp4 + tmp5
}
