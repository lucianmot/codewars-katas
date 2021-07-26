/* Java - Convert string to camel case.java
https://www.codewars.com/kata/517abf86da9663f1d2000003/train/java */

import java.lang.StringBuilder;
class Solution{

  static String toCamelCase(String s){
    s.replaceAll("[-]", "_");
    return "";
  }
}

// with regex replace - with _
// split string into array of string with _ as divider
// 1st word move to results
// loop for uppercase for remaining of words
// join string working array to results
