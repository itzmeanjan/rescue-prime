#pragma once
#include "ff.hpp"
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

}
