const { invoke } = window.__TAURI__.tauri;
async function register(username, password, email) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // console.log("Yes!");
  invoke("register", { username: username, password: password, email: email })
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
      console.error(error); // Handle error, display register failure message
      msg.textContent = "register failed: " + error;
    });
}
function hasANumb(i) {
  return /\d/.test(i); //return true if the input has a number
}

function hasASpecialCharacter(i) {
  return /[^A-Za-z0-9]/.test(i); //return true if input has a special character
}

function checkPassword(password, confirm_pass) {
  if (password != confirm_pass) {
    console.log("password", password);
    console.log("confirm_password", password);
    msg.textContent = "Passwords do not match";
  } else if (password.length <= 8) {
    msg.textContent = "Password does not meet requirements.";
  } else if (!hasANumb(password)) {
    msg.textContent = "Password does not include a number";
  } else if (!hasASpecialCharacter(password)) {
    msg.textContent = "Password does not include a special character";
  }else{
    return true;
  }
}

  window.addEventListener("DOMContentLoaded", () => {
    let username = document.querySelector("#reg_username");
    let password = document.querySelector("#reg_password");
    let confirm_pass = document.querySelector("#confirm_password");
    let email = document.querySelector("#email");
    let msg = document.querySelector("#msg");
    document.querySelector("#register-form").addEventListener("submit", (e) => {
      e.preventDefault();
      if (username.value && password.value && confirm_pass.value && email.value) {
          if(checkPassword(password.value, confirm_pass.value)){
            register(username.value, password.value, email.value);
          }else{
            msg.textContent = "Password must meet all requirements";
          }
      } else {
        console.log("username", username);
        console.log("password", password);
        console.log("confirm_password", password);
        console.log("email", email);
        msg.textContent = "Please fill in all fields";
      }
    });
  });


