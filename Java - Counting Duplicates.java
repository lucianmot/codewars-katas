// Java - Counting Duplicates
// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/

public class CountingDuplicates {
  
  public static int duplicateCount(String text) { 
    int duplicates = 0;
    
    text = text.toUpperCase();    
    
    for (char c = 'A'; c <= 'Z'; c++) {
      if (frequency (text, c) > 1) {
        duplicates++;
      }
    }
    
    for (char c = '0'; c <= '9'; c++) {
      if (frequency (text, c) > 1) {
        duplicates++;
      }
    }
    
    return duplicates;
  }
  
  public static int frequency (String text, char c) {
    
    int k = 0;
    
    for (int i = 0; i < text.length(); i++) {
      if (text.charAt(i) == c) {
        k++;
      }
    }
    
  return k;
  }
  
}
