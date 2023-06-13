#include <iostream>
#include "../target/cxxbridge/rslibcc/src/main.rs.h"

int main() {
  SomeNum s = { 10 };

  std::cout << s.num << "\n";

  s.num = 20;

  std::cout << s.num << "\n";
}
