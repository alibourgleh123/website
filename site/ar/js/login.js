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

        fetch("/login", {
            method: "POST",
            body: JSON.stringify({
              username: usernameInput.value,
              password: passwordInput.value,
              token: localStorage.getItem("access_token")
            }),
            headers: {
              "Content-type": "application/json; charset=UTF-8"
            }
          })
            .then((response) => response.json())
            .then((json) => {
                if (json.server_returned_an_error) {
                    showError("واجه خادمنا خطأً، حاول أن تسجل الدخول لاحقاً");
                    return;
                }
                if (json.access_granted) {
                    // TODO
                } else {
                    showError("اسم المستخدم أو كلمة المرور خاطئة");
                }
            });          
    });
});