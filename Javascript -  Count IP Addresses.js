// Javascript -  Count IP Addresses
// https://www.codewars.com/kata/526989a41034285187000de4/train/javascript 

function ipsBetween(start, end){
  start = start.split('.');
  end = end.split('.');
  
  let result = 0;
  for (let i=0; i<4; i++)
      result += (parseInt(end[i]) - parseInt(start[i]))*(Math.pow(256,3-i));    
  
  return result;  
}
