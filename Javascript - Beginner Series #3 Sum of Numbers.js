//Javascript - Beginner Series #3 Sum of Numbers.js
//https://www.codewars.com/kata/55f2b110f61eb01779000053/train/javascript

function getSum( a,b ) {
   if ( a === b ) {
     return a;
   }
  
  let s = 0;
  
   if ( a > b ) {
     a = a + b;
     b = a - b;
     a = a - b;
   }

  for (let i = a; i <= b; i++) {
    s+= i;
  }
  return s;
}
