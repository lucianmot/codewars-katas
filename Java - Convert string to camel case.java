/* Java - Convert string to camel case.java
https://www.codewars.com/kata/517abf86da9663f1d2000003/train/java */

import java.lang.StringBuilder;
class Solution{
  static String toCamelCase(String s){    
    s = s.replaceAll("-", "_");
    String [] a = s.split("_");
    
    s = "";
    s += a[0];
    
    for (int i=1; i<a.length; i++)      
      s += a[i].substring(0, 1).toUpperCase() + a[i].substring(1).toLowerCase();
    
    return s;
  }
}
