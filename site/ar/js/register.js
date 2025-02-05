document.addEventListener('DOMContentLoaded', () => {
    const usernameInput = document.getElementById('username');
    const passwordInput = document.getElementById('password');
    const registerButton = document.getElementById('registerButton');

    // TODO: uncomment this when releasing the website
    //usernameInput.focus();

    usernameInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            passwordInput.focus();
        }
    });

    passwordInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            e.preventDefault();
            registerButton.click();
        }
    });

    function showError(message) {
        const errorElement = document.getElementById('error-message');
        const errorSpan = errorElement.querySelector('span');
        errorSpan.textContent = message;
        errorElement.classList.remove('hidden');
    }

    function hideError() {
        const errorElement = document.getElementById('error-message');
        errorElement.classList.add('hidden');
    }

    document.addEventListener('click', function(event) {
        const errorElement = document.getElementById('error-message');
        const isClickInside = errorElement.contains(event.target);
        const isErrorVisible = !errorElement.classList.contains('hidden');

        if (isClickInside && isErrorVisible) {
            hideError();
        }
    });

    registerButton.addEventListener('click', () => {
        showError('أعاد الخادم خطأً: لا نقبل الوراعين هنا');
    });
});