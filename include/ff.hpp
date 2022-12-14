#pragma once
#include <cstdint>

// Prime Field ( i.e. F_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

// Prime Field Modulus
constexpr uint64_t Q = 0xffffffff00000001ul;

// An element of prime field F_q | q = 2^64 - 2^32 + 1, with arithmetic
// operations defined over it
struct ff_t
{
  uint64_t v = 0ul;

  inline constexpr ff_t(const uint64_t a = 0ul) { v = a % Q; }

  // Generate field element having canonical value 0
  static inline constexpr ff_t zero() { return ff_t{ 0ul }; }

  // Generate field element having canonical value 1
  static inline constexpr ff_t one() { return ff_t{ 1ul }; }
};

}
