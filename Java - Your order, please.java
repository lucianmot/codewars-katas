/* Java - Your order, please
https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/java */

public class Order {
  public static String order(String words) {
    if(words == "") {
      return "";
    }
    
    String[] wordsArray = words.split(" ");
    String[] results = new String[wordsArray.length];
    
    for (int i = 0; i < wordsArray.length; i++) {
        String word = wordsArray[i];
        String digit = word.replaceAll("\\D+","");
        System.out.println(digit);
        int location = Integer.parseInt(digit);
        results[location-1] = word;
    }
    
    String final_results = String.join(" ", results);
    
  return final_results;
  }
}
