#pragma once

#if defined __ARM_NEON
#include "ff.hpp"
#include <arm_neon.h>

// Prime Field ( i.e. Z_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

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
