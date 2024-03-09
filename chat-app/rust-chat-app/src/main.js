const { invoke } = window.__TAURI__.tauri;

let msg;
let username;
let password;
let response;
// let user_info;
let greet_msg;

async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("login", { username: username.value, password: password.value })
  .then(users => {
    if (users) {
      // Assuming users is the array of users, redirect or handle logged-in state
      window.location.replace("/home.html");
      console.log(users[0].username); // Log or use the users data as needed
      // greet_msg = document.querySelector("#greet-msg");
      // greet_msg.textContent = `Hello ${users[0].username}`;
      sessionStorage.setItem('username', users[0].username);
    }
  })
  .catch(error => {
    console.error(error); // Handle error, display login failure message
    // Optionally update the UI to reflect the error
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


