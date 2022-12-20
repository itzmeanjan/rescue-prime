# rescue-prime
Rescue Prime Hash Function

## Overview

Arithmetization friendly hash functions i.e. a hash function which works with prime field elements instead of working with raw bits/ bytes/ N -bit words as done by general-purpose hash functions such as SHA3-256, SHA256 or BLAKE3 etc., are very much used in STARK proof systems, due to the fact that their arithmetic circuit over prime field Z_q is much easier to prove. Rescue-Prime hash function is one such arithmetization friendly hash function, which is used by Winterfell STARK prover.

In Winterfell STARK prover, Rescue permutation is performed over prime field Z_q | q = $2^{64} - 2^{32} + 1$ i.e. the hash function exposes API of following form

- Input: N (>0) -many elements ∈ Z_q | q = $2^{64} - 2^{32} + 1$
- Output: 4 elements ∈ Z_q | q = $2^{64} - 2^{32} + 1$

Here I'm maintaining yet another implementation of Rescue Prime hash function over Z_q | q = $2^{64} - 2^{32} + 1$ which is conformant with Winterfell implementation. 

`rescue-prime` is a zero-dependency, header-only, C++ library which is easy to use. I've written both scalar & vectorized Rescue implementations. If target CPU has AVX2, it can be used to perform Rescue permutation faster.

> **Note**

> For understanding STARK https://aszepieniec.github.io/stark-anatomy

> **Note**

> Original Rescue specification https://ia.cr/2020/1143

> **Note**

> Read more about Rescue Prime in Winterfell [here](https://github.com/novifinancial/winterfell/tree/21173bdf3e552ca7662c7aa2d34515b084ae21b0/crypto)

> **Note**

> Find more about Winterfell https://github.com/novifinancial/winterfell

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

### On Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz ( **Scalar** implementation compiled with GCC )

```bash
2022-12-20T12:39:17+00:00
Running ./bench/a.out
Run on (128 X 1312.71 MHz CPU s)
CPU Caches:
  L1 Data 48 KiB (x64)
  L1 Instruction 32 KiB (x64)
  L2 Unified 1280 KiB (x64)
  L3 Unified 55296 KiB (x2)
Load Average: 0.22, 0.11, 0.04
-------------------------------------------------------------------------------------------------------------------------------------------------------------
Benchmark                                      Time             CPU   Iterations items_per_second max_exec_time (ns) median_exec_time (ns) min_exec_time (ns)
-------------------------------------------------------------------------------------------------------------------------------------------------------------
bench_rphash::permutation/manual_time      31497 ns        43352 ns        22225       31.7489k/s            36.866k               31.484k            31.359k
bench_rphash::hash/4/manual_time           32135 ns        36167 ns        21781       31.1185k/s            42.126k               32.123k            32.006k
bench_rphash::hash/8/manual_time           31813 ns        39758 ns        22003       31.4342k/s            33.936k               31.801k             31.68k
bench_rphash::hash/16/manual_time          63613 ns        79371 ns        11004         15.72k/s           100.307k               63.586k             63.39k
bench_rphash::hash/32/manual_time         127225 ns       158622 ns         5502       7.86011k/s           158.618k               127.17k           126.914k
bench_rphash::hash/64/manual_time         254419 ns       317107 ns         2751       3.93052k/s           268.945k               254.35k           253.972k
bench_rphash::hash/128/manual_time        508787 ns       634438 ns         1376       1.96546k/s           511.657k              508.659k           508.175k
```

### On Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz ( **AVX2** implementation compiled with GCC )

```bash
2022-12-20T12:40:12+00:00
Running ./bench/a.out
Run on (128 X 800.524 MHz CPU s)
CPU Caches:
  L1 Data 48 KiB (x64)
  L1 Instruction 32 KiB (x64)
  L2 Unified 1280 KiB (x64)
  L3 Unified 55296 KiB (x2)
Load Average: 0.16, 0.12, 0.04
-------------------------------------------------------------------------------------------------------------------------------------------------------------
Benchmark                                      Time             CPU   Iterations items_per_second max_exec_time (ns) median_exec_time (ns) min_exec_time (ns)
-------------------------------------------------------------------------------------------------------------------------------------------------------------
bench_rphash::permutation/manual_time      13741 ns        25476 ns        50939       72.7727k/s            19.977k               13.738k            13.667k
bench_rphash::hash/4/manual_time           13758 ns        17734 ns        50882       72.6836k/s            19.871k               13.753k             13.67k
bench_rphash::hash/8/manual_time           13752 ns        21609 ns        50898       72.7189k/s            24.249k               13.747k            13.676k
bench_rphash::hash/16/manual_time          27480 ns        43095 ns        25472       36.3905k/s             31.38k               27.472k            27.364k
bench_rphash::hash/32/manual_time          54943 ns        86061 ns        12741       18.2006k/s            64.596k               54.923k            54.779k
bench_rphash::hash/64/manual_time         109851 ns       172031 ns         6371       9.10323k/s           113.356k              109.824k           109.642k
bench_rphash::hash/128/manual_time        219686 ns       344632 ns         3186       4.55195k/s           223.527k               219.63k           219.317k
```

### On Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz ( **Scalar** implementation compiled with Clang )

```bash
2022-12-20T12:41:18+00:00
Running ./bench/a.out
Run on (128 X 1274.66 MHz CPU s)
CPU Caches:
  L1 Data 48 KiB (x64)
  L1 Instruction 32 KiB (x64)
  L2 Unified 1280 KiB (x64)
  L3 Unified 55296 KiB (x2)
Load Average: 0.11, 0.12, 0.05
-------------------------------------------------------------------------------------------------------------------------------------------------------------
Benchmark                                      Time             CPU   Iterations items_per_second max_exec_time (ns) median_exec_time (ns) min_exec_time (ns)
-------------------------------------------------------------------------------------------------------------------------------------------------------------
bench_rphash::permutation/manual_time      10723 ns        23808 ns        65285       93.2596k/s            17.495k               10.718k            10.636k
bench_rphash::hash/4/manual_time           10723 ns        15157 ns        65284       93.2552k/s            20.942k                10.72k             10.64k
bench_rphash::hash/8/manual_time           10725 ns        19487 ns        65275       93.2367k/s            16.557k               10.722k            10.642k
bench_rphash::hash/16/manual_time          21434 ns        38842 ns        32652       46.6538k/s            30.348k               21.428k            21.317k
bench_rphash::hash/32/manual_time          42832 ns        77525 ns        16343       23.3468k/s            48.002k               42.819k            42.645k
bench_rphash::hash/64/manual_time          85622 ns       154866 ns         8175       11.6793k/s                90k               85.596k            85.362k
bench_rphash::hash/128/manual_time        171201 ns       309544 ns         4089        5.8411k/s           174.509k              171.158k           170.811k
```

### On Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz ( **AVX2** implementation compiled with Clang )

```bash
2022-12-20T12:42:08+00:00
Running ./bench/a.out
Run on (128 X 1193.04 MHz CPU s)
CPU Caches:
  L1 Data 48 KiB (x64)
  L1 Instruction 32 KiB (x64)
  L2 Unified 1280 KiB (x64)
  L3 Unified 55296 KiB (x2)
Load Average: 0.17, 0.14, 0.06
-------------------------------------------------------------------------------------------------------------------------------------------------------------
Benchmark                                      Time             CPU   Iterations items_per_second max_exec_time (ns) median_exec_time (ns) min_exec_time (ns)
-------------------------------------------------------------------------------------------------------------------------------------------------------------
bench_rphash::permutation/manual_time       9890 ns        23015 ns        70776       101.114k/s            12.859k                9.887k             9.822k
bench_rphash::hash/4/manual_time            9900 ns        14329 ns        70708        101.01k/s            90.445k                9.895k             9.818k
bench_rphash::hash/8/manual_time            9899 ns        18657 ns        70717       101.019k/s            25.879k                9.896k             9.823k
bench_rphash::hash/16/manual_time          19768 ns        37182 ns        35407       50.5873k/s             23.27k               19.762k            19.648k
bench_rphash::hash/32/manual_time          39503 ns        74213 ns        17720       25.3145k/s            69.901k               39.491k            39.348k
bench_rphash::hash/64/manual_time          78971 ns       148260 ns         8865       12.6629k/s            83.093k               78.949k            78.775k
bench_rphash::hash/128/manual_time        157925 ns       296371 ns         4432       6.33214k/s            183.93k              157.873k           157.635k
```
