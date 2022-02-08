// Javascript -  Format a string of names like 'Bart, Lisa & Maggie'
// https://www.codewars.com/kata/53368a47e38700bd8300030d/train/javascript

function list(names){
  if (names.length == 1) {
    return names[0].name;
  } else if (names.length == 0) {
    return "";
  } else {
  var last_name = names.pop().name
  return names.map( x => x.name).join(", ") + " & " + last_name;
  }
}
