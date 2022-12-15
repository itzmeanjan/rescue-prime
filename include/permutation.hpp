#pragma once
#include "ff.hpp"

// Rescue Permutation over prime field Z_q, q = 2^64 - 2^32 + 1
//
// Constants are taken from
// https://github.com/novifinancial/winterfell/blob/437dc08/crypto/src/hash/rescue/rp64_256/mod.rs#L252-L269
namespace rescue {

// Capacity portion of Rescue permutation state begins at index 0
constexpr size_t CAPACITY_BEGINS = 0ul;

// Capacity portion of Rescue permutation state spans over first four elements
// of the state i.e. starting at index 0 and ending at index 3.
constexpr size_t CAPACITY = 4ul;

// Rate portion of Rescue permutation state begins at index 4
constexpr size_t RATE_BEGINS = CAPACITY_BEGINS + CAPACITY;

// Rate portion of Rescue permutation state spans over last eight elements of
// the state i.e. starting at index 4 and ending at index 11.
constexpr size_t RATE = 8ul;

// Rescue permutation state is 12 elements wide where each index holds an
// element ∈ Z_q
constexpr size_t STATE_WIDTH = CAPACITY + RATE;

// Digest portion of Rescue permutation state begins at index 4
constexpr size_t DIGEST_BEGINS = RATE_BEGINS;

// Digest portion of Rescue permutation state spans over first four elements of
// the rate portion i.e. starting at index 4 and ending at index 7.
constexpr size_t DIGEST_WIDTH = 4ul;

// Rescue permutation requires application of 7 rounds to target 128 -bit
// security with 40% security margin
constexpr size_t ROUNDS = 7ul;

}
