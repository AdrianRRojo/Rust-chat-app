const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", () => {
    //const username = sessionStorage.getItem('username');

    let password_msg = document.querySelector("#password_msg");
    let account_msg = document.querySelector("#account_msg");
    let delete_msg = document.querySelector("#delete_msg");
    let username = sessionStorage.getItem('username');
    
    const usernameDisplay = document.getElementById('account_username');

    if (usernameDisplay && username) {
        usernameDisplay.placeholder = `${username}`;
    }
    async function update_password(user_id, curr_password, new_password){
        invoke("update_password", { userId: parseInt(user_id, 10), currPassword: curr_password, newPassword: new_password })
            .then(password => {
                console.log(password);
                password_msg.textContent = "Success!";    
            })
            .catch(error => {
                console.error(error);
            });
    }
    async function delete_account(userId){
        invoke("delete_account", { userId: parseInt(userId, 10)})
            .then(delete_response => {
                console.log(delete_response);
                window.location.href = 'index.html';
                delete_msg.textContent = "Success!";           
            })
            .catch(error => {
                console.error(error);
            });
    }
    async function update_username(user_id, new_username){
        console.log(new_username);
        invoke("update_username", { userId: parseInt(user_id, 10), newUsername: new_username })
            .then(response => {
                console.log(response);
                account_msg.textContent = "Success!";    
            })
            .catch(error => {
                console.error(error);
                account_msg.textContent = error;
            });
    }
    document.querySelector("#account_info_form").addEventListener("submit", (e) => {
        e.preventDefault();

        let user_id = sessionStorage.getItem('userId');
        let new_username = document.querySelector("#account_username"); 

        if (user_id, new_username){
            if(new_username.value == username){
                account_msg.textContent = "New username cannot be the same as your previous username.";
            }else{
                console.log(new_username.value);
                update_username(user_id, new_username.value);
            }
        }
      });
      
      document.querySelector("#password_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');

        let user_id = sessionStorage.getItem('userId');
        let curr_password = document.querySelector("#account_curr_password"); 
        let new_password = document.querySelector("#account_new_password"); 
        let confirm_password= document.querySelector("#account_confirm_password");

        if (user_id && curr_password && new_password && confirm_password){
            // create_chat_room(chat_name.value, user_id);
            if(new_password.value == confirm_password.value){
                update_password(user_id, curr_password.value, new_password.value);
            }else{
                password_msg.textContent = "New passwords do not match";
            }
        }else{
            password_msg.textContent = "Please fill all fields";
        }
      });
      document.querySelector("#delete_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');
        let user_id = sessionStorage.getItem('userId');
        let delete_response = document.querySelector("#delete_input");
        // console.log(delete_response);
        if (user_id && delete_response){
            if(delete_response.value == "delete" || delete_response.value == "Delete" ){
                delete_account(user_id);
            }else{
                delete_msg.textContent = "Error: To delete your account please enter 'Delete'";
            }
        }
      });
      
})