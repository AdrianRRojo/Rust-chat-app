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
                    var element = document.createElement('h1');
                    element.textContent = `${chatsFromInvoke[i].name}`;
                    // console.log('test: ', chatsFromInvoke[i].name);
                    chatroomsContainer.appendChild(element); 
                }
            }
        })
        .catch(error => {
            console.error(error);
        });
}

window.addEventListener("DOMContentLoaded", () => {
    const username = sessionStorage.getItem('username');
    const usernameDisplay = document.getElementById('usernameDisplay');

    if (usernameDisplay && username) {
        usernameDisplay.textContent = `${username}`;
        load_chats();
    }
    
})
  