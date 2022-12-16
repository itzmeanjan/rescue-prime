#pragma once
#include "ff_avx.hpp"
#include <cassert>

// Test functional correctness of Rescue Prime implementation
namespace test_rphash {

// Test functional correctness of prime field ( i.e. Z_q ) operations, by
// running through multiple rounds ( see template parameter ) of execution of
// field operations on randomly sampled field element
template<const size_t rounds = 1024ul>
void
test_field_ops()
{
  static_assert(rounds > 0, "Round must not be = 0 !");

  std::random_device rd;
  std::mt19937_64 gen(rd());
  std::uniform_int_distribution<size_t> dis{ 0ul, 1ul << 20 };

  for (size_t i = 0; i < rounds; i++) {
    const auto a = ff::ff_t::random();
    const auto b = ff::ff_t::random();

    // addition, subtraction, negation
    const auto c = a - b;
    const auto d = -b;
    const auto e = a + d;

    assert(c == e);

    // multiplication, division, inversion
    const auto f = a * b;
    const auto g = f / b;

    if (b == ff::ff_t::zero()) {
      assert(g == ff::ff_t::zero());
    } else {
      assert(g == a);
    }

    const auto h = a.inv();
    const auto k = h * a;

    if (a == ff::ff_t::zero()) {
      assert(k == ff::ff_t::zero());
    } else {
      assert(k == ff::ff_t::one());
    }

    // exponentiation, multiplication
    const size_t exp = dis(gen);
    const auto l = a ^ exp;

    auto res = ff::ff_t::one();
    for (size_t j = 0; j < exp; j++) {
      res = res * a;
    }

    assert(res == l);
  }
}

#if defined __AVX2__

// Test that vectorized modulo addition over Z_q is implemented correctly
// by checking computed values against scalar implementation.
template<const size_t rounds = 256ul>
void
test_avx_mod_add()
{
  static_assert(rounds > 0, "Round must not be = 0 !");

  alignas(32) ff::ff_t arr0[4 * rounds];
  alignas(32) ff::ff_t arr1[4 * rounds];
  alignas(32) ff::ff_t computed_res[4 * rounds];
  alignas(32) ff::ff_t expected_res[4 * rounds];

  // generate some random Z_q elements
  for (size_t i = 0; i < 4 * rounds; i++) {
    arr0[i] = ff::ff_t::random();
    arr1[i] = ff::ff_t::random();
  }

  // compute modulo addition over Z_q, using scalar implementation
  for (size_t i = 0; i < 4 * rounds; i++) {
    expected_res[i] = arr0[i] + arr1[i];
  }

  // compute modulo addition over Z_q, using AVX2 implementation
  for (size_t i = 0; i < rounds; i++) {
    const size_t off = i * 4;

    const ff::ff_avx_t a{ arr0 + off };
    const ff::ff_avx_t b{ arr1 + off };

    const ff::ff_avx_t c = a + b;
    c.store(computed_res + off);
  }

  // finally ensure both implementations produce same result.
  for (size_t i = 0; i < 4 * rounds; i++) {
    assert(computed_res[i] == expected_res[i]);
  }
}

#endif

}
