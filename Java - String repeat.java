/* Java - String repeat
https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/java */

public class Solution {
    public static String repeatStr(final int repeat, final String string) {
        String results = "";
        for (int i = 0; i < repeat; i++) {
            results += string;
        }
        return results;
    }
}
