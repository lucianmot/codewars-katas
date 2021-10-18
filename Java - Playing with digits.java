/* Java - Playing with digits
https://www.codewars.com/kata/5552101f47fc5178b1000050/train/java */

public class DigPow {
  
  public static long digPow(int n, int p) {
    
    String temp = Integer.toString(n);
    int[] new_n = new int[temp.length()];
    for (int i = 0; i < temp.length(); i++)
      new_n[i] = temp.charAt(i) - '0';
    
    int big_n = 0;    
    for(int i = 0; i < temp.length(); i++)
      big_n += Math.pow(new_n[i], p+i);
    
    if(big_n % n != 0)
      return -1;
    
    return big_n / n;
  }
  
}
