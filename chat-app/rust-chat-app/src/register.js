const { invoke } = window.__TAURI__.tauri;
async function register() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    console.log("Yes!");
    // invoke("register", { username: username.value, password: password.value })
    // .then(users => {
    //   if (users) {
    //     //  console.log(users[0].id);
    //     window.location.replace("/home.html");
    //     // console.log(users[0].username);
    //     sessionStorage.setItem('username', users[0].username);
    //     // console.log(users[0].id);
    //     sessionStorage.setItem('userId', users[0].id);
        
    //   }
    // })
    // .catch(error => {
    //   console.error(error); // Handle error, display register failure message
    //   msg.textContent = "register failed: " + error;
    // });
  }
  window.addEventListener("DOMContentLoaded", () => {
   let username = document.querySelector("#reg_username");
   let password = document.querySelector("#reg_password");
   let confirm_pass = document.querySelector("#confirm_password");
   let email = document.querySelector("#email");
   let msg = document.querySelector("#msg");
   document.querySelector("#register-form").addEventListener("submit", (e) => {
        e.preventDefault();
        if(username.value && password.value && confirm_pass.value && email.value){
            if (password.value != confirm_pass.value){
                console.log("password", password);
                console.log("confirm_password", password);
                msg.textContent = "Passwords do not match";
            }else{
                register();
            }
        }else{
            console.log("username", username);
            console.log("password", password);
            console.log("confirm_password", password);
            console.log("email", email);
            msg.textContent = "Please fill in all fields";
        }
    });
});


//    if(username && password && confirm_pass && email){
//         if (password != confirm_pass){
//             msg.textContent = "Passwords do not match";
//         }else{
//             document.querySelector("#register-form").addEventListener("submit", (e) => {
//                 e.preventDefault();
//                 register();
//               });
//         }
//     }else{
//         console.log("username", username);
//         console.log("password", password);
//         console.log("confirm_password", password);
//         console.log("email", email);
//     }
//   });
  
  
  