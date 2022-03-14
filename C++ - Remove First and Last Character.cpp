// C++ - Remove First and Last Character
// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/cpp

/*
#include <string>
using namespace std; 

string sliceString (string str ) {
  str.erase(0,1);
  str.erase(str.size()-1);
  return str; 
}
*/

#include <string>
using namespace std; 

string sliceString (string str ){
  return str.substr(1, str.size() - 2); 
}
