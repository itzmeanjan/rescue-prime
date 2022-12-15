#include "test/test_ff.hpp"
#include "test/test_permutation.hpp"
#include <iostream>

int
main()
{
  test_rphash::test_field_ops<4096>();
  std::cout << "[test] Rescue Prime field arithmetic\n";

  test_rphash::test_alphas();
  test_rphash::test_permutation();
  std::cout << "[test] Rescue Permutation\n";

  return EXIT_SUCCESS;
}
