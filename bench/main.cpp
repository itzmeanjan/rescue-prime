#include "bench/bench_rescue_prime.hpp"
#include "benchmark/benchmark.h"

// Register for benchmarking Rescue permutation
BENCHMARK(bench_rphash::permutation)->UseManualTime();

BENCHMARK_MAIN();
