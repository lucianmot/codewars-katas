/* Java - Beginner Series #1 School Paperwork
https://www.codewars.com/kata/55f9b48403f6b87a7c0000bd/train/java */


public class Paper
{
  public static int paperWork(int n, int m) 
  {
    if ( n < 0 || m < 0 ) {
      return 0;
    } else {
      return n * m;
    }
  }
}
