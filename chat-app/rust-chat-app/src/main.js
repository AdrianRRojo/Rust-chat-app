const { invoke } = window.__TAURI__.tauri;

let msg;
let username;
let password;
let response;

async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  response = await invoke("login", { username: username.value, password: password.value });
  if(response){
    // msg.textContent = "Logged In";
    window.location.replace("/home.html");
  }else if(response == false){
    msg.textContent = "Failed to Log in."
  }
}

window.addEventListener("DOMContentLoaded", () => {
  username = document.querySelector("#username");
  password = document.querySelector("#password");
  msg = document.querySelector("#msg");
  document.querySelector("#login-form").addEventListener("submit", (e) => {
    e.preventDefault();
    login();
  });
});

