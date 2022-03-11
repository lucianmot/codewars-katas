// C++ - Reversed Strings
// https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/cpp

#include <string>
using namespace std ; 

string reverseString (string str ) {
  string new_str;

  for(int n = str.length()-1; n >= 0; n--){
    new_str.push_back(str[n]);
  }
  return new_str;
}
