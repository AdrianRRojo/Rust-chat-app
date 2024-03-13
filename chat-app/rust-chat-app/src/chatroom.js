const { invoke } = window.__TAURI__.tauri;


async function load_msgs(chat_id){
    invoke("load_msgs", {chatId: parseInt(chat_id, 10 )})
        .then(msgsFromChat =>{
            // console.log(msgsFromChat);
            if(msgsFromChat && msgsFromChat.length > 0){
                const chatroomsContainer = document.querySelector('#chatroom-content');
                for (let i = 0; i < msgsFromChat.length; i++){
                    var pTag = document.createElement('p');
                    var user = msgsFromChat[i].username;
                    var message = msgsFromChat[i].message;

                    let msg = `${user}: ${message}`;

                    pTag.innerText = msg;
                    pTag.setAttribute('id', 'chatMsgs');

                    chatroomsContainer.appendChild(pTag);

                }
            }else{
                console.log(msgsFromChat);
            }
        }).catch(error => {
            console.warn(error);
        }) 
}

async function send_msg(chat_id, user_id, user_msg){
    invoke("send_msg", {chatId: chat_id, userId: user_id, userMsg: user_msg })
        .then(msg =>{
            console.log(msg);
        }).catch(error => {
            console.warn(error);
        })
}

document.addEventListener('DOMContentLoaded', () => {
    const params = new URLSearchParams(window.location.search);
    const chatName = params.get('name');
    const chat_id = params.get('id');

    // Update the page title and chatroom name placeholder
    document.title = chatName; 
    document.getElementById('chatroom-name').textContent = chatName;
    document.getElementById('chatroom-content').textContent = `Welcome to ${chatName}! Chat ID is ${chat_id}.`;

    load_msgs(chat_id);
    
    document.querySelector("#msg_form").addEventListener("submit", (e) => {
        e.preventDefault();
        // console.log('testing');
        let user_id = sessionStorage.getItem('userId');
        let user_msg = document.querySelector('#user_msg')
        if (user_id && user_msg){
            send_msg(chat_id, user_id, user_msg.value);
            // console.log(access_code.value);
        }else{
            console.warn("Chat: ",chat_id, "User: ", user_id, "Msg: ", user_msg.value);
        }
      });

});
