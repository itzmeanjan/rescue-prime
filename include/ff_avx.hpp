#pragma once
#include "ff.hpp"

#if defined __AVX2__
#include <immintrin.h>

// Prime Field ( i.e. Z_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

// Given two 256 -bit registers, each holding four 64 -bit unsigned integers,
// this routine computes whether a >= b.
//
// Resulting 256 -bit register holds four 64 -bit limbs, each setting all of its
// bits to 1, if a >= b, otherwise all 64 of those bits ( of that specific limb
// ) is set to 0.
static inline __m256i
gte(const __m256i a, const __m256i b)
{
  // To determine whether a > b | a, b ∈ [0, 2^64), it's enough to compare high
  // 32 -bits of a, b.
  //
  // I've to choose this path due to the fact that `_mm256_cmpgt_epi64`
  // intrinsic treats each 64 -bit limb to be a signed 64 -bit integer, which
  // effectively rounds off operands and that ends up producing wrong comparison
  // result, in a lot of cases.

  const auto t0 = _mm256_srli_epi64(a, 32);
  const auto t1 = _mm256_srli_epi64(b, 32);

  const auto t2 = _mm256_cmpgt_epi64(t0, t1); // is > ?
  const auto t3 = _mm256_cmpeq_epi64(a, b);   // is = ?
  const auto t4 = _mm256_or_si256(t2, t3);    // is >= ?

  return t4;
}

// Given a 256 -bit register, holding four 64 -bit unsigned integers, this
// routine converts each of those four limbs to their canonical representation
// in prime field Z_q i.e. returned register holds four elements ∈ Z_q.
static inline __m256i
reduce(const __m256i a)
{
  const auto q = _mm256_set1_epi64x(ff::Q);

  const auto t0 = gte(a, q);
  const auto t1 = _mm256_and_si256(t0, q);
  const auto t2 = _mm256_sub_epi64(a, t1);

  return t2;
}

// Four elements of prime field Z_q | q = 2^64 - 2^32 + 1, stored in a 256 -bit
// AVX2 register, loaded *only* from 32 -bytes aligned memory address ( see
// constructor ), defining modular {addition, multiplication} over it.
struct ff_avx_t
{
  __m256i v;

  // Assign a 256 -bit register
  inline constexpr ff_avx_t(const __m256i a) { v = a; }

  // Load four 64 -bit unsigned integers from memory into a 256 -bit register.
  //
  // Ensure that starting memory address is 32 -bytes aligned, otherwise it'll
  // result in a segmentation fault.
  inline ff_avx_t(const ff::ff_t* const arr)
  {
    v = _mm256_load_si256((__m256i*)arr);
  }

  // Given two 256 -bit registers, each holding 4 prime field Z_q elements, this
  // routine performs element wise addition over Z_q and returns result in
  // canonical form i.e. each 64 -bit result limb must ∈ Z_q.
  inline ff_avx_t operator+(const ff_avx_t& rhs) const
  {
    const auto u256 = _mm256_set1_epi64x(UINT64_MAX);

    const auto t0 = _mm256_add_epi64(this->v, rhs.v);
    const auto t1 = _mm256_sub_epi64(u256, rhs.v);

    const auto t2 = _mm256_srli_epi64(this->v, 32);
    const auto t3 = _mm256_srli_epi64(t1, 32);
    const auto t4 = _mm256_cmpgt_epi64(t2, t3);

    const auto t5 = _mm256_srli_epi64(t4, 32);
    const auto t6 = _mm256_add_epi64(t0, t5);

    const auto t7 = reduce(t6);

    return ff_avx_t{ t7 };
  }

  // Stores four prime field Z_q elements ( kept in a 256 -bit register ) into
  // 32 -bytes aligned memory s.t. starting memory address is provided. If
  // starting memory address is not aligned to 32 -bytes boundary, it'll result
  // in a segmentation fault.
  inline void store(ff::ff_t* const arr) const
  {
    _mm256_store_si256((__m256i*)arr, this->v);
  }
};

}

#endif
