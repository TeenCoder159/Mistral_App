const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function getResponse() {
  if (greetInputEl.value != "") {
    greetMsgEl.innerHTML = await invoke("get", {
      input: greetInputEl.value,
    });
  } else {
    return "Please provide an input";
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#prompt-input");
  greetMsgEl = document.querySelector("#output-msg");
  document.querySelector("#input-form").addEventListener("submit", (e) => {
    e.preventDefault();
    getResponse();
  });
});
