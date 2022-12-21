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

  // Stores two prime field Z_q elements ( kept in a 128 -bit register )
  inline void store(ff::ff_t* const arr) const
  {
    vst1q_u64(reinterpret_cast<uint64_t*>(arr), this->v);
  }
};

}

#endif
