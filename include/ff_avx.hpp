#pragma once
#include "ff.hpp"

#if defined __AVX2__
#include <immintrin.h>

// Prime Field ( i.e. Z_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

// Four elements of prime field Z_q | q = 2^64 - 2^32 + 1, stored in a 256 -bit
// AVX2 register, loaded *only* from 32 -bytes aligned memory address ( see
// constructor ), defining modular {addition, multiplication} over it.
struct ff_avx_t
{
  __m256i v;

  inline constexpr ff_avx_t(const ff::ff_t* const arr)
  {
    v = _mm256_load_si256((__m256i*)arr);
  }
};

}

#endif
