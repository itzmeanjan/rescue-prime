#include "test/test_ff.hpp"
#include <iostream>

int
main()
{
  test_rphash::test_field_ops<4096>();
  std::cout << "[test] Rescue Prime field arithmetic\n";

  return EXIT_SUCCESS;
}