#pragma once
#include "permutation.hpp"
#include <cassert>

// Test functional correctness of Rescue Prime implementation
namespace test_rphash {

// Check if α and inv_α are correctly valued, which are used for applying S-Box
// and Inverse S-Box during Rescue Permutation
void
test_alphas()
{
  const auto v = ff::ff_t::random();
  const auto v_alpha = v ^ rescue::ALPHA;
  const auto v_inv_alpha = v_alpha ^ rescue::INV_ALPHA;

  assert(v == v_inv_alpha);
}

// Check Rescue permutation using known answer test
//
// Test vector taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/tests.rs#L47-L83
void
test_permutation()
{
  constexpr ff::ff_t expected[rescue::STATE_WIDTH]{
    10809974140050983728ul, 6938491977181280539ul,  8834525837561071698ul,
    6854417192438540779ul,  4476630872663101667ul,  6292749486700362097ul,
    18386622366690620454ul, 10614098972800193173ul, 7543273285584849722ul,
    9490898458612615694ul,  9030271581669113292ul,  10101107035874348250ul,
  };
  ff::ff_t state[rescue::STATE_WIDTH]{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 };

  rescue::permute(state);

  for (size_t i = 0; i < rescue::STATE_WIDTH; i++) {
    assert(state[i] == expected[i]);
  }
}

}
