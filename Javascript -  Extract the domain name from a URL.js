// Javascript -  Extract the domain name from a URL.js
// https://www.codewars.com/kata/514a024011ea4fb54200004b/train/javascript  

function domainName(url){
  return url.replace('http://', '').replace('https://', '').replace('www.', '').split('.')[0];
}
