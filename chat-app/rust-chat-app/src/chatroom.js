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


document.addEventListener('DOMContentLoaded', () => {
    // Parse the URL query parameters
    const params = new URLSearchParams(window.location.search);
    const chatName = params.get('name');
    const chat_id = params.get('id');

    // Update the page title and chatroom name placeholder
    document.title = chatName; // Set the browser tab title to the chatroom name
    document.getElementById('chatroom-name').textContent = chatName;

    // Here you would typically make a call to your backend to fetch chatroom data using the chatId
    // For this example, let's just update the chatroom content placeholder
    document.getElementById('chatroom-content').textContent = `Welcome to ${chatName}! Chat ID is ${chat_id}.`;

    load_msgs(chat_id);
    // TODO: Fetch and display the chatroom's actual content using chatId
});
