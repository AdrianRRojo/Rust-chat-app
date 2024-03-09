const { invoke } = window.__TAURI__.tauri;

let greetMsgEl;
let username;
let password;

async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("login", { username: username.value, password: password.value });
}

window.addEventListener("DOMContentLoaded", () => {
  username = document.querySelector("#username");
  password = document.querySelector("#password");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#login-form").addEventListener("submit", (e) => {
    e.preventDefault();
    login();
  });
});

