// C++ - Convert boolean values to strings 'Yes' or 'No'.
// https://www.codewars.com/kata/53369039d7ab3ac506000467/train/cpp

/*
#include <string>

std::string bool_to_word(bool value) {
  return (value == true) ? "Yes" : "No";
}
*/

#include <string>

std::string bool_to_word(bool value) {
  return value ? "Yes" : "No";
}
