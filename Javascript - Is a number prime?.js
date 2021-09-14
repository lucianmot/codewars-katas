// Javascript - Is a number prime?
// https://www.codewars.com/kata/5262119038c0985a5b00029f/train/javascript

function isPrime(num) {
//   if (num <= 1)
//     return false;
//   for (let i = 3; i < num/2; i + 2)
//     if (num % i === 0) 
//       return false;
//   return true;
  
  for  ( let i = 2; i < num; i ++)
    if (num % i === 0) return false;
  return num > 1;
}
