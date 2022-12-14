#pragma once
#include <bit>
#include <cstddef>
#include <cstdint>
#include <random>

// Prime Field ( i.e. Z_q ) Arithmetic | q = 2^64 - 2^32 + 1
namespace ff {

// Prime Field Modulus
constexpr uint64_t Q = 0xffffffff00000001ul;

// An element of prime field Z_q | q = 2^64 - 2^32 + 1, with arithmetic
// operations defined over it
struct ff_t
{
  uint64_t v = 0ul;

  // Given a 64 -bit unsigned integer, this routine returns an element of Z_q,
  // where its value is kept in canonical representation.
  inline constexpr ff_t(const uint64_t a = 0ul)
  {
    const bool flg = a >= Q;
    const uint64_t t = a - flg * Q;
    v = t;
  }

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

    return ff_t{ t2 };
  }

  // Negation over prime field, such that both input operand and output are in
  // canonical form
  inline constexpr ff_t operator-() const
  {
    const uint64_t t = Q - this->v;
    return ff_t{ t };
  }

  // Subtraction over prime field, such that both input operands and output are
  // in canonical form
  inline constexpr ff_t operator-(const ff_t& rhs) const
  {
    const ff_t t = -rhs;
    return *this + t;
  }

  // Multiplication over prime field, such that both input operands and output
  // are in canonical form
  inline constexpr ff_t operator*(const ff_t& rhs) const
  {
    // Multiply lhs and rhs s.t. high and low 64 -bit limbs of 128 -bit result,
    // becomes accessible
    const uint64_t lhs_hi = this->v >> 32;
    const uint64_t lhs_lo = this->v & 0xfffffffful;

    const uint64_t rhs_hi = rhs.v >> 32;
    const uint64_t rhs_lo = rhs.v & 0xfffffffful;

    const uint64_t hi = lhs_hi * rhs_hi;                    // high bits
    const uint64_t mid = lhs_hi * rhs_lo + lhs_lo * rhs_hi; // mid bits
    const uint64_t lo = lhs_lo * rhs_lo;                    // low bits

    const uint64_t mid_hi = mid >> 32;          // high 32 -bits of mid
    const uint64_t mid_lo = mid & 0xfffffffful; // low 32 -bits of mid

    const uint64_t t0 = lo >> 32;
    const uint64_t t1 = t0 + mid_lo;
    const uint64_t carry = t1 >> 32;

    // res = lhs * rhs | res is a 128 -bit number
    //
    // assert res = (res_hi << 64) | res_lo
    const uint64_t res_hi = hi + mid_hi + carry;
    const uint64_t res_lo = lo + (mid_lo << 32);

    const uint64_t c = res_hi & 0xfffffffful;
    const uint64_t d = res_hi >> 32;

    const uint64_t t2 = res_lo - d;
    const bool flg0 = res_lo < d;
    const uint64_t t3 = static_cast<uint64_t>(-static_cast<uint32_t>(flg0));
    const uint64_t t4 = t2 - t3;

    const uint64_t t5 = (c << 32) - c;
    const uint64_t t6 = t4 + t5;
    const bool flg1 = t4 > UINT64_MAX - t5;
    const uint64_t t7 = static_cast<uint64_t>(-static_cast<uint32_t>(flg1));
    const uint64_t t8 = t6 + t7;

    return ff_t{ t8 };
  }

  // Raises an element of Z_q to N -th power, over prime field, using
  // exponentiation by repeated squaring rule. Note, both input element and
  // output elements are kept in their canonical form.
  //
  // Adapted from
  // https://github.com/itzmeanjan/kyber/blob/3cd41a5/include/ff.hpp#L224-L246
  inline constexpr ff_t operator^(const size_t n) const
  {
    ff_t base = *this;

    const ff_t br[]{ ff_t::one(), base };
    ff_t res = br[n & 0b1ul];

    const size_t zeros = std::countl_zero(n);
    const size_t till = 64ul - zeros;

    for (size_t i = 1; i < till; i++) {
      base = base * base;

      const ff_t br[]{ ff_t::one(), base };
      res = res * br[(n >> i) & 0b1ul];
    }

    return res;
  }

  // Multiplicative inverse over prime field
  //
  // Say input is a & return value of this function is b, then
  //
  // assert (a * b) % q == 1
  //
  // When input a = 0, multiplicative inverse can't be computed, hence return
  // value is 0.
  //
  // Adapted from
  // https://github.com/itzmeanjan/ff-gpu/blob/89c9719/ff_p.cpp#L103-L121
  inline constexpr ff_t inv() const { return *this ^ (Q - 2); }

  // Division over prime field Z_q
  inline constexpr ff_t operator/(const ff_t& rhs) const
  {
    return (*this) * rhs.inv();
  }

  // Check equality of canonical values of two elements ∈ Z_q
  inline constexpr bool operator==(const ff_t& rhs) const
  {
    return !static_cast<bool>(this->v ^ rhs.v);
  }

  // Check inequality of canonical values of two elements ∈ Z_q
  inline constexpr bool operator!=(const ff_t& rhs) const
  {
    return !(*this == rhs);
  }

  // Generate a random element ∈ Z_q
  static inline ff_t random()
  {
    std::random_device rd;
    std::mt19937_64 gen(rd());
    std::uniform_int_distribution<uint64_t> dis{ 0ul, Q - 1ul };

    return ff_t{ dis(gen) };
  }
};

}
