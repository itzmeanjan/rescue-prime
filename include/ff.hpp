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

  // Addition over prime field, both input operands and result to stay in
  // canonical form
  inline constexpr ff_t operator+(const ff_t& rhs) const
  {
    const uint64_t t0 = this->v + rhs.v;

    const bool flg0 = this->v > UINT64_MAX - rhs.v;
    const uint64_t t1 = static_cast<uint64_t>(-static_cast<uint32_t>(flg0));
    const uint64_t t2 = t0 + t1;

    const bool flg1 = t2 >= Q;
    const uint64_t t3 = t2 - flg1 * Q;

    return ff_t{ t3 };
  }

  // Negation over prime field, such that both input operand and output are in
  // canonical form
  inline constexpr ff_t operator-() const
  {
    const uint64_t t = Q - this->v;
    return ff_t{ t };
  }

  // Subtraction over prime field, such that both input operands and output are
  // in canonical
  // form
  inline constexpr ff_t operator-(const ff_t& rhs) const
  {
    const ff_t t = -rhs;
    return *this + t;
  }
};

}
