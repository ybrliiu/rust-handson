"use strict";

const js = import("./rust_handson");

js.then(js => {
  const ul = document.createElement('ul');
  document.getElementsByTagName('body')[0].appendChild(ul);
  ['World', 'fuuge', 'hogeee'].forEach(str => {
    const li = document.createElement('li');
    li.innerHTML = js.sha256(str);
    ul.appendChild(li);
  });
});
