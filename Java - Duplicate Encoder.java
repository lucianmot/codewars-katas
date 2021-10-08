/* Java - Duplicate Encoder
https://www.codewars.com/kata/54b42f9314d9229fd6000d9c/train/java */

public class DuplicateEncoder {
	static String encode(String word){
    
    String solution = "";
    word = word.toUpperCase();
    
    for (int i = 0; i < word.length(); i++) {      
      if( frequency(word, word.charAt(i)) > 1 ){
        solution += ")";
      }
      else{
        solution += "(";
      }
    }
    
    return solution;
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
