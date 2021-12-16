## simd-rescue-prime-example

Using `simd-rescue-prime` library crate for computing Rescue Prime hash using `hash_elements` and `merge` functions.

Read [here](https://github.com/itzmeanjan/simd-rescue-prime/blob/0cbb6e5ccd622c44462ff57fefa91bbcdfcae1c4/README.md#L51-L63) for understanding what does these two functions do.

## Usage

Compile/ build while leveraging CPU specific vector instructions

```bash
RUSTFLAGS="-C target-feature=+avx2" cargo run --release
```

I'm using 

```bash
rustc 1.59.0-nightly (532d2b14c 2021-12-03)
```

because `portable-simd` is an experimental feature and only available on Rust nightly standard library.
