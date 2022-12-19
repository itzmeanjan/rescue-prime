# rescue-prime
Rescue Prime Hash Function

## Prerequisites

- A C++ compiler, with C++20 standard library such as `g++`/ `clang++`

```bash
$ clang++ --version
Ubuntu clang version 14.0.0-1ubuntu1
Target: aarch64-unknown-linux-gnu
Thread model: posix
InstalledDir: /usr/bin

$ g++ --version
g++ (Ubuntu 11.2.0-19ubuntu1) 11.2.0
```

- System development utilities such as `make`, `cmake` & `git`

```bash
$ make --version
GNU Make 4.3

$ cmake --version
cmake version 3.22.1

$ git --version
git version 2.34.1
```

- For benchmarking Rescue Prime implementation on CPU systems, you'll need to have `google-benchmark` globally installed. I found [this](https://github.com/google/benchmark/tree/da652a7#installation) guide useful.

## Testing

For ensuring functional correctness and compatibility with Winterfell implementation of Rescue Prime Hashing over Z_q | q = $2^{64} - 2^{32} + 1$, issue

> **Note** Find Winterfell implementation of Rescue Prime [here](https://github.com/novifinancial/winterfell/tree/21173bdf3e552ca7662c7aa2d34515b084ae21b0/crypto#rescue-hash-function-implementation)

> **Note** Rescue Permutation test vectors are adapted from [here](https://github.com/novifinancial/winterfell/blob/21173bdf3e552ca7662c7aa2d34515b084ae21b0/crypto/src/hash/rescue/rp64_256/tests.rs)

```bash
make # tests scalar implementation

[test] Rescue Prime field arithmetic
[test] Rescue Permutation
```

If your target CPU has AVX2 features, try testing that implementation by issuing

```bash
AVX2=1 make # tests AVX2 implementation

[test] Rescue Prime field arithmetic
[test] Vectorized Rescue Prime field arithmetic
[test] Rescue Permutation
```

## Benchmarking

For benchmarking 

- Rescue Permutation over Z_q | q = $2^{64} -2^{32} + 1$
- Z_q element hasher

issue following

```bash
make benchmark # benchmarks scalar implementation
```

If your target CPU has AVX2 features, you may want to benchmark that implementation by issuing

```bash
AVX2=1 make # benchmarks AVX2 implementation
```

> **Note**

> Benchmarking expects presence of google-benchmark library in global namespace ( so that it can be found by the compiler ).

> **Warning**

> Because most of the CPUs employ dynamic frequency boosting technique, when benchmarking routines, you may want to disable CPU frequency scaling by following [this](https://github.com/google/benchmark/blob/da652a7/docs/user_guide.md#disabling-cpu-frequency-scaling) guide.

### On Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz ( **Scalar** implementation compiled with Clang )

```bash
2022-12-19T21:37:33+04:00
Running ./bench/a.out
Run on (8 X 2400 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB
  L1 Instruction 32 KiB
  L2 Unified 256 KiB (x4)
  L3 Unified 6144 KiB
Load Average: 1.99, 1.89, 1.87
-------------------------------------------------------------------------------------------------------------------------------------------------------------
Benchmark                                      Time             CPU   Iterations items_per_second max_exec_time (ns) median_exec_time (ns) min_exec_time (ns)
-------------------------------------------------------------------------------------------------------------------------------------------------------------
bench_rphash::permutation/manual_time      11711 ns       137521 ns        59644       85.3871k/s           118.625k               11.562k            11.346k
bench_rphash::hash/4/manual_time           11718 ns        53749 ns        59847       85.3373k/s             83.78k               11.568k            11.373k
bench_rphash::hash/8/manual_time           11715 ns        95581 ns        59714       85.3587k/s            95.865k               11.578k            11.369k
bench_rphash::hash/16/manual_time          23341 ns       191033 ns        29877       42.8427k/s            141.08k               23.088k            22.717k
bench_rphash::hash/32/manual_time          46656 ns       381455 ns        14991       21.4337k/s           215.707k               46.097k            45.422k
bench_rphash::hash/64/manual_time          93244 ns       761240 ns         7503       10.7246k/s            305.16k               92.089k            90.817k
bench_rphash::hash/128/manual_time        186940 ns      1526762 ns         3750       5.34931k/s           395.294k              184.094k           181.659k
```

### On Intel(R) Core(TM) i5-8279U CPU @ 2.40GHz ( **AVX2** implementation compiled with Clang )

```bash
2022-12-19T21:39:09+04:00
Running ./bench/a.out
Run on (8 X 2400 MHz CPU s)
CPU Caches:
  L1 Data 32 KiB
  L1 Instruction 32 KiB
  L2 Unified 256 KiB (x4)
  L3 Unified 6144 KiB
Load Average: 0.96, 1.59, 1.76
-------------------------------------------------------------------------------------------------------------------------------------------------------------
Benchmark                                      Time             CPU   Iterations items_per_second max_exec_time (ns) median_exec_time (ns) min_exec_time (ns)
-------------------------------------------------------------------------------------------------------------------------------------------------------------
bench_rphash::permutation/manual_time       7576 ns       133394 ns        92596       131.988k/s            99.574k                7.467k             7.341k
bench_rphash::hash/4/manual_time            7569 ns        49631 ns        92266       132.122k/s            81.625k                7.471k             7.343k
bench_rphash::hash/8/manual_time            7589 ns        91627 ns        91952        131.77k/s            83.479k                7.474k             7.348k
bench_rphash::hash/16/manual_time          15058 ns       182693 ns        46561       66.4116k/s           127.783k               14.858k            14.646k
bench_rphash::hash/32/manual_time          29973 ns       365103 ns        23340       33.3633k/s           158.505k               29.592k            29.229k
bench_rphash::hash/64/manual_time          59811 ns       729436 ns        11739       16.7192k/s           255.018k               59.043k            58.412k
bench_rphash::hash/128/manual_time        119664 ns      1461539 ns         5861       8.35671k/s           291.594k              117.946k            116.75k
```
