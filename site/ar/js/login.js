document.addEventListener('DOMContentLoaded', () => {
    const usernameInput = document.getElementById('username');
    const passwordInput = document.getElementById('password');
    const loginButton = document.getElementById('loginButton');

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
            loginButton.click();
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

    let toggleErrorMessageHiding = 0;

    document.addEventListener('click', function(event) {
        const errorElement = document.getElementById('error-message');

        if (!errorElement) return;

        const isClickInside = errorElement.contains(event.target);
        const isErrorVisible = !errorElement.classList.contains('hidden');

        if (isClickInside && isErrorVisible) {
            toggleErrorMessageHiding++;

            if (toggleErrorMessageHiding >= 2) {
                hideError();
                toggleErrorMessageHiding = 0;
            }
        } else {
            toggleErrorMessageHiding = 0;
        }
    });


    loginButton.addEventListener('click', () => {
        if (!localStorage.getItem("access_token")) {
            localStorage.setItem("access_token", crypto.randomUUID());
        }

        if (usernameInput.value == "") {
            showError("لم تعطنا اسم مستخدم!");
            return;
        }

        if (passwordInput.value == "") {
            showError("لم تعطنا كلمة مرور!");
            return;
        }   
        fetch('/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                username: usernameInput.value,
                password: passwordInput.value,
                token: localStorage.getItem("access_token")
            })
        })
        .then(response => {
            if (response.redirected) {
                // If the server redirects, follow the redirect
                window.location.replace(response.url);
            } else {
                return response.json();
            }
        })
        .then(data => {
            if (data && !data.access_granted) {
                showError("اسم المستخدم أو كلمة المرور خاطئة");
            }
        })
        .catch(error => {
            showError(`حدث خطأ:\n${error}`);
        });
    });
});