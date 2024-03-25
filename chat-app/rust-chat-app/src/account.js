const { invoke } = window.__TAURI__.tauri;

window.addEventListener("DOMContentLoaded", () => {
    //const username = sessionStorage.getItem('username');

    let password_msg = document.querySelector("#password_msg");
    let account_msg = document.querySelector("#account_msg");
    let delete_msg = document.querySelector("#delete_msg");

    async function update_password(userId, curr_password, new_password){
        invoke("update_password", { userId: userId, curr_password: curr_password, new_password: new_password })
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
                delete_msg.textContent = "Success!";           
            })
            .catch(error => {
                console.error(error);
            });
    }

    document.querySelector("#password_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');

        let userId = sessionStorage.getItem('userId');
        let curr_password = document.querySelector("#account_curr_password"); 
        let new_password = document.querySelector("#account_new_password"); 
        let confirm_password= document.querySelector("#account_confirm_password");

        if (userId && curr_password && new_password && confirm_password){
            // create_chat_room(chat_name.value, user_id);
            if(new_password != confirm_password){
                password_msg.textContent = "New passwords do not match";
            }else{
                update_password(userId, curr_password, new_password);
            }
        }else{
            password_msg.textContent = "Please fill all fields";
        }
      });

      document.querySelector("#delete_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');
        let user_id = sessionStorage.getItem('userId');
        let delete_response = document.querySelector("delete_input");

        if (user_id && delete_response){
            if(delete_response.value != "delete" || delete_response.value != "Delete" ){
                delete_msg.textContent = "Error: To delete your account please enter 'Delete'";
            }else{
                delete_account(user_id);
            }
        }
      });
      
})