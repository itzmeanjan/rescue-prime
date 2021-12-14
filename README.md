# simd-rescue-prime
Portable SIMD powered Rescue Prime Hash

## Motivation

For sometime, I've been exploring Zk-STARK friendly Rescue Prime Hash function and writing following implementations, majorly for harnessing power of multi/ many core accelerator devices, such as CPU, GPU etc.

- [Data Parallel SYCL/ DPC++](https://github.com/itzmeanjan/ff-gpu/blob/9c57cb13e4b2d96a084da96d558fe3d4707bfcb7/rescue_prime.cpp)
- [Vectorized using OpenCL](https://github.com/itzmeanjan/vectorized-rescue-prime/blob/614500dd1f271e4f8badf1305c8077e2532eb510/kernel.cl#L345-L474)

This time I decided to implement Rescue Prime Hash function on 64-bit prime field `F(2^64 - 2^32 + 1)`, using _experimental_ `portable_simd` feature, which recently landed in Rust Nightly.

> More about it [here](http://github.com/rust-lang/rust/issues/86656).

> Notice, this crate [overrides](https://github.com/itzmeanjan/simd-rescue-prime/blob/9e83eb579a6e7666ae33d2c86524d8e287e7f1ca/rust-toolchain) default Rust toolchain version to use `portable_simd` feature.

This implementation takes quite a lot of motivation from [this OpenCL kernel](https://github.com/itzmeanjan/vectorized-rescue-prime/blob/614500dd1f271e4f8badf1305c8077e2532eb510/kernel.cl), where I implemented Rescue Prime Hash using OpenCL vector intrinsics.

Resue Prime Hash state is represented using a SIMD vector with 16 lanes, where each element is of type `u64` i.e. `Simd<u64; 16>` . Originally hash state is 12 elements wide, which is why last 4 lanes of hash state is padded with zeros and those places don't hold any useful values. Here I'm applying 7 rounds during Rescue permutation, following [this](https://github.com/novifinancial/winterfell/blob/4eeb4670387f3682fa0841e09cdcbe1d43302bf3/crypto/src/hash/rescue/rp64_256/mod.rs#L27-L29). All arithmetics are performed following rules of 64-bit prime field `F(2^64 - 2^32 + 1)`. Finally produced output takes first 4 elements of hash state after whole input is absorbed into state --- output is 4-field elements wide i.e. 32-bytes.

## Usage

- Make sure you've Rust toolchain installed. Note, this crate will override your default toolchain version for using `std::simd` library, which is behind `portable_simd` feature gate.
- Run test cases

```bash
cargo test
```

- Run benchmark on `hash_elements` and `merge` functions

```bash
cargo bench
```

- You'll find usage examples for both `hash_elements` and `merge` functions [here](https://github.com/itzmeanjan/simd-rescue-prime/blob/dcdebc35762a0dffcfce3278c2b8a8f892058809/src/main.rs#L14-L16)

## Benchmark Results

I'm using Cargo's experimental benchmark runner for benchmarking following two Rescue Prime Hash functions

- `hash_element`
- `merge`

One thing to note about aforementioned two functions, given an input like

```python
input = [0, 1, 2, 3, 4, 5, 6, 7]
```

following assertion must not fail

```python
assert hash_elements(input, ...) == merge(input, ...)
```

In simple terms, `merge` function merges two Rescue Prime digests into single digest, which is 4 field elements wide.

> When running both [benchmarks](https://github.com/itzmeanjan/simd-rescue-prime/blob/dcdebc35762a0dffcfce3278c2b8a8f892058809/src/rescue_prime.rs#L569), I use 8 field elements wide input.

> Following table denotes how long does it take to execute single round of `hash_elements`/ `merge` function invocation, on single core of specific CPU model.

CPU | `hash_elements` | `merge`
--- | --- | ---
Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz | 58,968 ns | 58,470 ns
Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz | 36,409 ns | 36,602 ns
Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz | 38,114 ns | 37,963 ns
Cortex-A57 | 214,118 ns | 213,401 ns
