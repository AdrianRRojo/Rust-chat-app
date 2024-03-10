const { invoke } = window.__TAURI__.tauri;
// let msg;
async function load_chats() {
    let userId = sessionStorage.getItem('userId');
    // console.log(userId);
    invoke("load_chats", { userId: parseInt(userId, 10) })
        .then(chatsFromInvoke => {
            if (chatsFromInvoke && chatsFromInvoke.length > 0) {
                const chatroomsContainer = document.querySelector('.chatrooms');
                for (let i = 0; i < chatsFromInvoke.length; i++) {
                    var text1 = document.createElement('h1');
                    let text = text1.textContent = `${chatsFromInvoke[i].name}`;
                    // chatroomsContainer.appendChild(text);
                    let newlink = document.createElement('a');
                    newlink.innerHTML = text;
                    newlink.setAttribute('title', text);
                    newlink.setAttribute('href', `/chat?=${text}/${chatsFromInvoke[i].id}`);
                    chatroomsContainer.appendChild(newlink);
                }
            }
        })
        .catch(error => {
            console.error(error);
        });
}

async function create_chat_room(chat_name){
    console.log("chat_name: ", chat_name);
    invoke('create_chat_room', {name: chat_name})
        .then(chats => {
            console.log("chats: ", chats);
        }).catch(error => {
            console.warn(error);
        })
}
window.addEventListener("DOMContentLoaded", () => {
    const username = sessionStorage.getItem('username');
    const usernameDisplay = document.getElementById('usernameDisplay');

    if (usernameDisplay && username) {
        usernameDisplay.textContent = `${username}`;
        load_chats();
    }
    document.querySelector("#create_chat_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');
        let user_id = sessionStorage.getItem('userId');
        let chat_name = document.querySelector("#chat_name");
        if (user_id && chat_name){
            create_chat_room(chat_name.value);
        }
      });
})
