/* Java - Reversed Strings 
https://www.codewars.com/kata/5168bb5dfe9a00b126000018/train/java */

public class Kata {

  public static String solution(String str) {    
    
    StringBuilder str2 = new StringBuilder();
    str2.append(str);
    str2.reverse();
    System.out.println(str2);
    
    return str2.toString();
  }

}
