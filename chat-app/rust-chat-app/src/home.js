 const { invoke } = window.__TAURI__.tauri;
// let msg;
async function load_chats() {
    let userId = sessionStorage.getItem('userId');
    // console.log(userId);
    invoke("load_chats", { userId: parseInt(userId, 10) })
        .then(chatsFromInvoke => {
            console.log(chatsFromInvoke);
            if (chatsFromInvoke && chatsFromInvoke.length > 0) {
                const chatroomsContainer = document.querySelector('.chatrooms');
                for (let i = 0; i < chatsFromInvoke.length; i++) {
                    var text1 = document.createElement('h1');
                    let text = text1.textContent = `${chatsFromInvoke[i].name}`;
                    // chatroomsContainer.appendChild(text);
                    let newlink = document.createElement('a');
                    newlink.innerHTML = text;
                    newlink.setAttribute('id', 'chatroom_links');
                    // newlink.setAttribute('href', `/charoom.html?name=${text}&id=${chatsFromInvoke[i].id}`);
                    newlink.href = `/chatroom.html?name=${text}&id=${chatsFromInvoke[i].id}`;
                    chatroomsContainer.appendChild(newlink);
                }
            }
        })
        .catch(error => {
            console.error(error);
        });
}

async function create_chat_room(chat_name, user_id){
    console.log("chat_name: ", chat_name);
    invoke('create_chat_room', {name: chat_name, userId: user_id})
        .then(chats => {
            console.log("chats: ", chats);
            location.reload();
        }).catch(error => {
            console.warn(error);
        })
}
async function join_chat_room(user_id, access_code){
    console.log("code JCR: ", access_code);
    console.log("UserID JCR: ", user_id);
    invoke('join_chat_room', {userId: parseInt(user_id, 10), accessCode: access_code})
        .then(chats => {
            console.log("chats: ", chats);
            location.reload();
        }).catch(error => {
            console.warn(error);
        })
}
window.addEventListener("DOMContentLoaded", () => {
    const username = sessionStorage.getItem('username');
    // const usernameDisplay = document.getElementById('usernameDisplay');

    if (username) {
        // usernameDisplay.textContent = `${username}`;
        // load_chats();
    }

    document.querySelector("#create_chat_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');

        let user_id = sessionStorage.getItem('userId');
        let chat_name = document.querySelector("#chat_name");
        let access_code = document.querySelector("#access_code");
        if (user_id && chat_name){
            create_chat_room(chat_name.value, user_id);
        }
        if (user_id && access_code){
            // join_chat_room(user_id, access_code.value);
            console.log(access_code.value);
        }
      });
      document.querySelector("#join_chat_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');
        let user_id = sessionStorage.getItem('userId');
        let access_code = document.querySelector("#access_code");
        if (user_id && access_code){
            join_chat_room(user_id, access_code.value);
            // console.log(access_code.value);
        }
      });
      
})
