use std::simd::{u64x4, SimdPartialOrd};

pub const ULONG_MAX: u64 = 0xffffffffffffffff;
pub const UINT_MAX: u64 = 0xffffffff;
pub const MOD: u64 = 0xffffffff00000001;

#[inline]
fn mul_hi_(a: u64x4, b: u64x4) -> u64x4 {
    let a_lo = a & u64x4::splat(UINT_MAX);
    let a_hi = a >> u64x4::splat(32u64);
    let b_lo = b & u64x4::splat(UINT_MAX);
    let b_hi = b >> u64x4::splat(32u64);

    let a_x_b_hi = a_hi * b_hi;
    let a_x_b_mid = a_hi * b_lo;
    let b_x_a_mid = b_hi * a_lo;
    let a_x_b_lo = a_lo * b_lo;

    let tmp0 = a_x_b_mid & u64x4::splat(UINT_MAX);
    let tmp1 = b_x_a_mid & u64x4::splat(UINT_MAX);
    let tmp2 = tmp0 + tmp1 + (a_x_b_lo >> u64x4::splat(32u64));
    let carry_bit = tmp2 >> u64x4::splat(32u64);

    a_x_b_hi + (a_x_b_mid >> u64x4::splat(32u64)) + (b_x_a_mid >> u64x4::splat(32u64)) + carry_bit
}

#[inline]
fn vec_mul_ff_p64_(a: u64x4, b: u64x4) -> u64x4 {
    let ab = a * b;
    let cd = mul_hi_(a, b);
    let c = cd & u64x4::splat(UINT_MAX);
    let d = cd >> u64x4::splat(32u64);

    let tmp0 = ab - d;
    let under0 = ab.simd_lt(d);
    let tmp1 = under0.select(u64x4::splat(1u64), u64x4::splat(0u64)) * u64x4::splat(UINT_MAX);
    let tmp2 = tmp0 - tmp1;

    let tmp3 = (c << u64x4::splat(32u64)) - c;

    let tmp4 = tmp2 + tmp3;
    let over0 = tmp2.simd_gt(u64x4::splat(ULONG_MAX) - tmp3);
    let tmp5 = over0.select(u64x4::splat(1u64), u64x4::splat(0u64)) * u64x4::splat(UINT_MAX);

    tmp4 + tmp5
}

#[inline]
pub fn vec_mul_ff_p64(a: [u64x4; 3], b: [u64x4; 3]) -> [u64x4; 3] {
    [
        vec_mul_ff_p64_(a[0], b[0]),
        vec_mul_ff_p64_(a[1], b[1]),
        vec_mul_ff_p64_(a[2], b[2]),
    ]
}

#[inline]
pub fn vec_add_ff_p64_(a: u64x4, b: u64x4) -> u64x4 {
    let t0 = b.simd_ge(u64x4::splat(MOD));
    let t1 = b - u64x4::splat(MOD);
    let b_ok = t0.select(t1, b);

    let tmp0 = a + b_ok;
    let over0 = a.simd_gt(u64x4::splat(ULONG_MAX) - b_ok);
    let tmp1 = over0.select(u64x4::splat(1u64), u64x4::splat(0u64)) * u64x4::splat(UINT_MAX);

    let tmp2 = tmp0 + tmp1;
    let over1 = tmp0.simd_gt(u64x4::splat(ULONG_MAX) - tmp1);
    let tmp3 = over1.select(u64x4::splat(1u64), u64x4::splat(0u64)) * u64x4::splat(UINT_MAX);

    tmp2 + tmp3
}

#[inline]
pub fn vec_add_ff_p64(a: [u64x4; 3], b: [u64x4; 3]) -> [u64x4; 3] {
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

    fn random_vector() -> u64x4 {
        let mut a = Vec::with_capacity(4);
        for _ in 0..4 {
            a.push(rand::random::<u64>() % MOD);
        }

        let arr: [u64; 4] = a.try_into().unwrap();
        u64x4::from_array(arr)
    }

    #[test]
    fn test_ff_mul_1_() {
        let a = [random_vector(), random_vector(), random_vector()];
        let b = vec_mul_ff_p64(a, [u64x4::splat(1u64); 3]);

        assert_eq!((b[0] % u64x4::splat(MOD)).to_array(), a[0].to_array());
        assert_eq!((b[1] % u64x4::splat(MOD)).to_array(), a[1].to_array());
        assert_eq!((b[2] % u64x4::splat(MOD)).to_array(), a[2].to_array());
    }

    #[test]
    fn test_ff_mul_2_() {
        let a: u64x4 = u64x4::splat(3u64);
        let b: u64x4 = u64x4::splat(5u64);
        assert_eq!(
            (vec_mul_ff_p64_(a, b) % u64x4::splat(MOD)).to_array(),
            [15u64; 4]
        );
    }

    #[test]
    fn test_ff_mul_3_() {
        let a = [u64x4::from_array([MOD - 1; 4]); 3];
        let b = [u64x4::from_array([2u64; 4]); 3];
        let c = [u64x4::from_array([4u64; 4]); 3];

        let res_0 = vec_mul_ff_p64(a, a);
        assert_eq!((res_0[0] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
        assert_eq!((res_0[1] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
        assert_eq!((res_0[2] % u64x4::splat(MOD)).to_array(), [1u64; 4]);

        let res_1 = vec_mul_ff_p64(a, b);
        assert_eq!((res_1[0] % u64x4::splat(MOD)).to_array(), [MOD - 2; 4]);
        assert_eq!((res_1[1] % u64x4::splat(MOD)).to_array(), [MOD - 2; 4]);
        assert_eq!((res_1[2] % u64x4::splat(MOD)).to_array(), [MOD - 2; 4]);

        let res_2 = vec_mul_ff_p64(a, c);
        assert_eq!((res_2[0] % u64x4::splat(MOD)).to_array(), [MOD - 4; 4]);
        assert_eq!((res_2[1] % u64x4::splat(MOD)).to_array(), [MOD - 4; 4]);
        assert_eq!((res_2[2] % u64x4::splat(MOD)).to_array(), [MOD - 4; 4]);
    }

    #[test]
    fn test_ff_mul_4_() {
        let a = [u64x4::from_array([(MOD + 1) / 2; 4]); 3];
        let b = [u64x4::from_array([2u64; 4]); 3];

        let res = vec_mul_ff_p64(a, b);
        assert_eq!((res[0] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
        assert_eq!((res[1] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
        assert_eq!((res[2] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
    }

    #[test]
    fn test_ff_add_0_() {
        let a = [random_vector(), random_vector(), random_vector()];
        let res = vec_add_ff_p64(a, [u64x4::splat(0u64); 3]);

        assert_eq!((res[0] % u64x4::splat(MOD)).to_array(), a[0].to_array());
        assert_eq!((res[1] % u64x4::splat(MOD)).to_array(), a[1].to_array());
        assert_eq!((res[2] % u64x4::splat(MOD)).to_array(), a[2].to_array());
    }

    #[test]
    fn test_ff_add_1_() {
        let a = [u64x4::from_array([2u64; 4]); 3];
        let b = [u64x4::from_array([3u64; 4]); 3];
        let res = vec_add_ff_p64(a, b);

        assert_eq!((res[0] % u64x4::splat(MOD)).to_array(), [5u64; 4]);
        assert_eq!((res[1] % u64x4::splat(MOD)).to_array(), [5u64; 4]);
        assert_eq!((res[2] % u64x4::splat(MOD)).to_array(), [5u64; 4]);
    }

    #[test]
    fn test_ff_add_2_() {
        let a = [u64x4::from_array([MOD - 1; 4]); 3];
        let b = [u64x4::splat(1u64); 3];
        let c: [u64x4; 3] = [u64x4::splat(2u64); 3];

        let res_0 = vec_add_ff_p64(a, b);
        assert_eq!((res_0[0] % u64x4::splat(MOD)).to_array(), [0u64; 4]);
        assert_eq!((res_0[1] % u64x4::splat(MOD)).to_array(), [0u64; 4]);
        assert_eq!((res_0[2] % u64x4::splat(MOD)).to_array(), [0u64; 4]);

        let res_1 = vec_add_ff_p64(a, c);
        assert_eq!((res_1[0] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
        assert_eq!((res_1[1] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
        assert_eq!((res_1[2] % u64x4::splat(MOD)).to_array(), [1u64; 4]);
    }

    #[test]
    fn test_ff_add_3_() {
        let a = [u64x4::from_array([MOD - 1; 4]); 3];
        let b = [u64x4::from_array([0xffffffffu64; 4]); 3];

        let res = vec_add_ff_p64(a, b);
        assert_eq!((res[0] % u64x4::splat(MOD)).to_array(), [0xfffffffeu64; 4]);
        assert_eq!((res[1] % u64x4::splat(MOD)).to_array(), [0xfffffffeu64; 4]);
        assert_eq!((res[2] % u64x4::splat(MOD)).to_array(), [0xfffffffeu64; 4]);
    }
}
