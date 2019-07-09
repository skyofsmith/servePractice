// This file is required by the index.html file and will
// be executed in the renderer process for that window.
// All of the Node.js APIs are available in this process.
console.log('window:', window);
setInterval(() => {
  let time = document.querySelector('#time');
  let now = new Date();
  time.innerHTML = `${now.getFullYear()}年${now.getMonth() + 1}月${now.getDate()}日`;
}, 500);