document.addEventListener('DOMContentLoaded', () => {
    // Parse the URL query parameters
    const params = new URLSearchParams(window.location.search);
    const chatName = params.get('name');
    const chatId = params.get('id');

    // Update the page title and chatroom name placeholder
    document.title = chatName; // Set the browser tab title to the chatroom name
    document.getElementById('chatroom-name').textContent = chatName;

    // Here you would typically make a call to your backend to fetch chatroom data using the chatId
    // For this example, let's just update the chatroom content placeholder
    document.getElementById('chatroom-content').textContent = `Welcome to ${chatName}! Chat ID is ${chatId}.`;

    // TODO: Fetch and display the chatroom's actual content using chatId
});
