/* Java - Grasshopper - Summation
https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/java */


public class GrassHopper {

    public static int summation(int n) {
            int results = 0;
            for (int i = 0; i <= n; i++) {
                results += i;
            }
        return results;
    }
}
