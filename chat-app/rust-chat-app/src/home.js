window.addEventListener("DOMContentLoaded", () => {
    const username = sessionStorage.getItem('username');
    const usernameDisplay = document.getElementById('usernameDisplay');

    if (usernameDisplay && username) {
        usernameDisplay.textContent = `Hello ${username}!`;
    }
})
  