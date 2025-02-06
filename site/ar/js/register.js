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
        hideError();
        // Regenerate token everytime the button is clicked so that no 2 accounts
        // will have the same token.
        localStorage.setItem("access_token", crypto.randomUUID());


        if (usernameInput.value == "") {
            showError("لم تعطنا اسم مستخدم!");
            return;
        }

        if (passwordInput.value == "") {
            showError("لم تعطنا كلمة مرور!");
            return;
        }

        fetch("/register", {
            method: "POST",
            headers: {
                "Content-type": "application/json"
            },
            body: JSON.stringify({
              username: usernameInput.value,
              password: passwordInput.value,
              token: localStorage.getItem("access_token")
            })
          })
        .then((response) => {
            if (response.redirected) {
                // If the server redirects, follow the redirect
                window.location.replace(response.url);
            } else {
                return response.json();
            }
        })
        .then((json) => {
            if (json.server_returned_an_error) {
                let error;
                switch(json.error_code) {
                    case 1:
                        error = "اسم المستخدم غير صالح، يجب أن يكون على الأقل مكوناً من 3 أحرف";
                        break;
                    case 2:
                        error = "كلمة المرور غير صالحة، يجب أن تكون على الأقل مكونةً من 3 أحرف";
                        break;
                    case 3:
                        error = "التذكرة المحلية غير صالحة، هذه المشكلة لا علاقة لك بها لذلك تواصل معنا لحلها";
                        break;
                    case 4: 
                        error = "اسم المستخدم مأخوذ";
                        break;
                    case 5: 
                        error = "حدث خطأ بقاعدة البيانات، هذه المشكلة لا علاقة لك بها لذلك تواصل معنا لحلها";
                        break;
                    default:
                        error = "واجهنا خطئاً غير معروف"
                        break;
                }
                showError(error);
                return;
            }
            })
        .catch(error => {
            showError(`حدث خطأ:\n${error}`)
        });
    });            
});
