#include "test/test_ff.hpp"
#include "test/test_permutation.hpp"
#include <iostream>

int
main()
{
  test_rphash::test_field_ops();
  std::cout << "[test] Rescue Prime field arithmetic\n";

#if defined __AVX2__ && USE_AVX2 != 0

  test_rphash::test_avx_mod_add();
  test_rphash::test_u64_full_mul();
  test_rphash::test_avx_mod_mul();
  std::cout << "[test] Vectorized Rescue Prime field arithmetic\n";

#endif

  test_rphash::test_alphas();
  test_rphash::test_permutation();
  std::cout << "[test] Rescue Permutation\n";

  return EXIT_SUCCESS;
}
