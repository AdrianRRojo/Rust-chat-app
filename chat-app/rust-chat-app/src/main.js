const { invoke } = window.__TAURI__.tauri;

let msg;
let username;
let password;
let response;
let greet_msg;

async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("login", { username: username.value, password: password.value })
  .then(users => {
    if (users) {
      //  console.log(users[0].id);
      window.location.replace("/home.html");
      // console.log(users[0].username);
      sessionStorage.setItem('username', users[0].username);
      // console.log(users[0].id);
      sessionStorage.setItem('userId', users[0].id);
      
    }
  })
  .catch(error => {
    console.error(error); // Handle error, display login failure message
    msg.textContent = "Login failed: " + error;
  });
}
window.addEventListener("DOMContentLoaded", () => {
  username = document.querySelector("#username");
  password = document.querySelector("#password");
  msg = document.querySelector("#msg");
  greet_msg = document.querySelector("#greet-msg");
  document.querySelector("#login-form").addEventListener("submit", (e) => {
    e.preventDefault();
    login();
  });
});


