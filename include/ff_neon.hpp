#pragma once

#if defined __ARM_NEON
#include "ff.hpp"
#include <arm_neon.h>

// Prime Field ( i.e. Z_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

// Given a 128 -bit register, holding two 64 -bit unsigned integers, this
// routine converts each of those two limbs into their canonical representation
// in prime field Z_q, by reducing using prime modulus Q = 2^64 - 2^32 + 1
//
// This routine does exactly what
// https://github.com/itzmeanjan/rescue-prime/blob/22b7aa5/include/ff.hpp#L62-L64
// does, but it reduces two field elements at a time.
static inline uint64x2_t
reduce(const uint64x2_t a)
{
  const auto q = vdupq_n_u64(ff::Q);

  const auto t0 = vcgeq_u64(a, q);
  const auto t1 = vandq_u64(t0, q);
  const auto t2 = vsubq_u64(a, t1);

  return t2;
}

// Two elements of prime field Z_q | q = 2^64 - 2^32 + 1, stored in a 128 -bit
// Neon register, defining modular {addition, multiplication} over it.
struct ff_neon_t
{
  uint64x2_t v;

  // Assign a 128 -bit register
  inline constexpr ff_neon_t(const uint64x2_t a) { v = a; }

  // Load two consecutive 64 -bit unsigned integers from memory into a 128 -bit
  // register.
  inline ff_neon_t(const ff::ff_t* const arr)
  {
    v = vld1q_u64(reinterpret_cast<const uint64_t*>(arr));
  }

  // Given two 128 -bit registers, each holding two prime field Z_q elements,
  // this routine performs element wise addition over Z_q and returns result in
  // canonical form i.e. each 64 -bit result limb must ∈ Z_q.
  //
  // This routine does what
  // https://github.com/itzmeanjan/rescue-prime/blob/22b7aa5/include/ff.hpp#L73-L84
  // does, only difference is that instead of working on single element at a
  // time, it works on two of them.
  inline ff_neon_t operator+(const ff_neon_t& rhs) const
  {
    const auto u64x2 = vdupq_n_u64(UINT64_MAX);

    const auto t0 = vaddq_u64(this->v, rhs.v);

    const auto t1 = vsubq_u64(u64x2, rhs.v);
    const auto t2 = vcgtq_u64(this->v, t1);
    const auto t3 = vshrq_n_u64(t2, 32);
    const auto t4 = vaddq_u64(t0, t3);

    const auto t5 = reduce(t4);
    return ff_neon_t{ t5 };
  }

  // Stores two prime field Z_q elements ( kept in a 128 -bit register )
  inline void store(ff::ff_t* const arr) const
  {
    vst1q_u64(reinterpret_cast<uint64_t*>(arr), this->v);
  }
};

}

#endif
