// Javascript - Valid Parentheses.js
// https://www.codewars.com/kata/52774a314c2333f0a7000688/train/javascript

function validParentheses(parens) {
  let state = 0;

  for (let c = 0; c < parens.length; c++) {
    parens[c] === ")" ? state-- : state++;
    if (state < 0) return false;
  }

  return state === 0;
}
