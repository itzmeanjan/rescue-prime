#pragma once
#include "permutation.hpp"

// Rescue Prime Hashing over prime field Z_q, q = 2^64 - 2^32 + 1
namespace rescue_prime {

// Given N ( > 0 ) -many Z_q elements as input, this routine computes Rescue
// prime digest of four Z_q elements i.e. 32 -bytes wide.
//
// This implementation is adapted from
// https://github.com/novifinancial/winterfell/blob/21173bd/crypto/src/hash/rescue/rp64_256/mod.rs#L223-L256
static inline void
hash(const ff::ff_t* const __restrict in,
     const size_t ilen,
     ff::ff_t* const __restrict out)
{
  ff::ff_t state[rescue::STATE_WIDTH]{};
  state[rescue::CAPACITY_BEGINS] = ff::ff_t{ ilen };

  const size_t blk_cnt = ilen >> 3;
  const size_t off = blk_cnt << 3;
  const size_t rm_elms = ilen - off;

  for (size_t i = 0; i < blk_cnt; i++) {
    const size_t ioff = i << 3;

    std::memcpy(state + rescue::RATE_BEGINS, in + ioff, rescue::RATE << 3);
    rescue::permute(state);
  }

  std::memcpy(state + rescue::RATE_BEGINS, in + off, rm_elms << 3);
  if (rm_elms > 0) {
    rescue::permute(state);
  }

  std::memcpy(out, state + rescue::DIGEST_BEGINS, rescue::DIGEST_WIDTH << 3);
}

}
