// C++ - Even or Odd
// https://www.codewars.com/kata/53da3dbb4a5168369a0000fe/train/cpp

#include <string>

std::string even_or_odd(int number) {
  std::string result = (number % 2 == 0) ? "Even" : "Odd";
  return result;
}
