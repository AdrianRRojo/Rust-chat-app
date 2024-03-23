window.addEventListener("DOMContentLoaded", () => {
    const username = sessionStorage.getItem('username');
    // const usernameDisplay = document.getElementById('usernameDisplay');

    // if (username) {
        // usernameDisplay.textContent = `${username}`;
        console.log(username);
    // }





    // document.querySelector("#create_chat_form").addEventListener("submit", (e) => {
    //     e.preventDefault();
    //     // console.log('testing');

    //     let user_id = sessionStorage.getItem('userId');
    //     let chat_name = document.querySelector("#chat_name");
    //     let access_code = document.querySelector("#access_code");
    //     if (user_id && chat_name){
    //         create_chat_room(chat_name.value, user_id);
    //     }
    //     if (user_id && access_code){
    //         // join_chat_room(user_id, access_code.value);
    //         console.log(access_code.value);
    //     }
    //   });
    //   document.querySelector("#join_chat_form").addEventListener("submit", (e) => {
    //     e.preventDefault();
    //     // console.log('testing');
    //     let user_id = sessionStorage.getItem('userId');
    //     let access_code = document.querySelector("#access_code");
    //     if (user_id && access_code){
    //         join_chat_room(user_id, access_code.value);
    //         // console.log(access_code.value);
    //     }
    //   });
      
})