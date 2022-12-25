#pragma once

#if defined __AVX512F__
#include "ff.hpp"
#include <immintrin.h>

// Prime Field ( i.e. Z_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

// Eight elements of the prime field Z_q | q = 2^64 - 2^32 + 1, stored in a 512
// -bit AVX512 register, loaded *only* from 64 -bytes aligned memory address (
// see constructor ), defining modular {addition, multiplication} over it, used
// for implementing Rescue permutation.
struct ff_avx512_t
{
  __m512i v;

  // Assign a 512 -bit register
  inline constexpr ff_avx512_t(const __m512i a) { v = a; }

  // Load eight 64 -bit unsigned integers from memory into a 512 -bit register.
  //
  // Ensure that starting memory address is 64 -bytes aligned, otherwise it'll
  // result in a segmentation fault.
  inline ff_avx512_t(const ff::ff_t* const arr) { v = _mm512_load_epi64(arr); }

  // Stores eight prime field Z_q elements ( kept in a 512 -bit register ) into
  // 64 -bytes aligned memory s.t. starting memory address is provided.
  //
  // If starting memory address is not aligned to 64 -bytes boundary, it'll
  // result in a segmentation fault.
  inline void store(ff::ff_t* const arr) const
  {
    _mm512_store_epi64(arr, this->v);
  }
};

}

#endif
