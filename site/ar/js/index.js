document.querySelectorAll('.country-code').forEach((input) => {
  input.addEventListener('input', function (e) {
    let value = e.target.value.replace(/[^0-9()]/g, '');

    // Limit to 3 digits
    value = value.slice(0, 3);
  
    // Add the "+" prefix
    e.target.value = value ? `+${value}` : '';
  });
});

document.querySelectorAll('.phone-number').forEach((input) => {
  input.addEventListener('input', function (e) {
      // Remove non-numeric characters
      let value = e.target.value.replace(/[^0-9()]/g, '');

      // Add space after the closing parentheses if it doesn't exist
      if (value.includes(')') && !value.includes(') ')) {
          value = value.replace(')', ') ');
      }

      e.target.value = value;
  });
});

// The consultation form error handlers
function showConsultationError(message) {
  const errorElement = document.getElementById("consultation-error-message");
  const errorSpan = errorElement.querySelector("span");
  errorSpan.textContent = message;
  errorElement.classList.remove("hidden");
}

function hideConsultationError() {
  const errorElement = document.getElementById("consultation-error-message");
  errorElement.classList.add("hidden");
}

function showConsultationInfo(message) {
  document.getElementById('consultation-info-message-span').textContent = message;
  document.getElementById('consultation-info-message').classList.remove("hidden");
}

function hideConsultationInfo() {
  document.getElementById('consultation-info-message').classList.add("hidden");
}

let consultationFilesUploadFailed = false;

async function sendConsultationRequest() {
    let uuid = "";
    let fetch_returned_an_error = false;

    await fetch("/send_consultation", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            targeted_doctor: doctorNameVar,
            name: document.getElementById("consultation-patients-name").value,
            sur_name: document.getElementById("consultation-patients-surname").value,
            email: document.getElementById("consultation-patients-email").value,
            phone_number: `${document.getElementById("consultation-country-code").value} ${document.getElementById("consultation-patients-phone-number").value}`,
            issue: document.getElementById("consultation-patients-issue").value,
        }),
    })
        .then((response) => {
            if (!response.ok) {
                fetch_returned_an_error = true;
            }
            return response.json();
        })
        .then((response) => {
            if (fetch_returned_an_error) {
                hideConsultationInfo();
                showConsultationError(response.error);
                return;
            }
            uuid = response.uuid;
            
            if (uuid) {
                uploadConsultationFiles(uuid);
            }

            if (consultationFilesUploadFailed) {
                showConsultationError("فشلت عملية رفع ملفاتك");
                consultationFilesUploadFailed = false;
            } else {
                document.querySelectorAll('.consultation-value-reset-after-submit').forEach((input) => {
                  input.value = '';
                });
                hideConsultationError();
                showConsultationInfo("تم إرسال طلبك بنجاح، سيتم التواصل والتنسيق معك عن طريق طرق التواصل التي وضعتها");
            }
        })
        .catch((error) => {
            showConsultationError(error);
        });
}

function uploadConsultationFiles(uuid) {
    const files = document.getElementById("ConsultationFiles").files;

    if (files.length === 0) {
        return;
    }

    document.getElementById("progressContainer").innerHTML = "";

    Array.from(files).forEach((file, index) => {
        uploadSingleFile(file, index, uuid);
    });
}

function uploadSingleFile(file, index, uuid) {
    const formData = new FormData();
    formData.append("file", file);

    const progressBar = document.createElement("progress");
    progressBar.value = 0;
    progressBar.max = 100;
    progressBar.id = `progress-${index}`;
    progressBar.style.display = "block";
    progressBar.style.maxWidth = "fit-content";
    progressBar.style.margin = "0 auto";

    const statusText = document.createElement("span");
    statusText.id = `status-${index}`;
    statusText.innerText = `يتم رفع ${file.name}...`;
    statusText.style.display = "block";
    statusText.style.maxWidth = "fit-content";
    statusText.style.margin = "0 auto";

    document.getElementById("progressContainer").appendChild(document.createElement("br"));
    document.getElementById("progressContainer").appendChild(statusText);
    document.getElementById("progressContainer").appendChild(progressBar);

    fetch(`/uploadconsultationfile?filename=${encodeURIComponent(file.name)}&uuid=${encodeURIComponent(uuid)}`, {
        method: "POST",
        body: formData,
    })
        .then((response) => {
            if (!response.ok) {
                throw new Error(`HTTP error! Status: ${response.status}`);
            }
        })
        .then(() => {
            document.getElementById(`status-${index}`).innerText = `✅ تم رفع: ${file.name}  `;
        })
        .catch((error) => {
            document.getElementById(`status-${index}`).innerText = `❌ فشل رفع: ${file.name}  `;
            consultationFilesUploadFailed = true;
            console.error("Error:", error);
        });

    // Simulate progress update
    simulateProgress(progressBar);
}

function simulateProgress(progressBar) {
    let progress = 0;
    const interval = setInterval(() => {
        progress += 10;
        progressBar.value = progress;
        if (progress >= 100) clearInterval(interval);
    }, 300);
}


// The form errors handlers
// too lazy to rename it
function showError(message) {
  const errorElement = document.getElementById("error-message");
  const errorSpan = errorElement.querySelector("span");
  errorSpan.textContent = message;
  errorElement.classList.remove("hidden");
}

function hideError() {
  const errorElement = document.getElementById("error-message");
  errorElement.classList.add("hidden");
}

let toggleErrorMessageHiding = 0;

document.addEventListener("click", function (event) {
  const errorElement = document.getElementById("error-message");

  if (!errorElement) return;

  const isClickInside = errorElement.contains(event.target);
  const isErrorVisible = !errorElement.classList.contains("hidden");

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

function disableScroll() {
  document.body.style.overflowY = "hidden"; // Prevents page scrolling
}

function enableScroll() {
  document.body.style.overflowY = ""; // Restores scrolling
}

let doctorNameVar = "";

function showConsultationPopup() {
  hideConsultationInfo();
  hideConsultationError();
  document.querySelectorAll('.html-reset-at-open').forEach((div) => {
    div.innerHTML = '';
  });
  document.querySelectorAll('.consultation-value-reset-after-submit').forEach((input) => {
    input.value = '';
  });
  document.getElementById("consultation-popup").style.display = "flex";
  document.getElementById('consultation-doctor-name').innerText = `عند ${doctorNameVar}`;
  document.getElementById("closeConsultationPopup").addEventListener("click", function () {
    document.getElementById("consultation-popup").style.display = "none";
  });
  disableScroll();
}

function handleConsultation() { 
  doctorNameVar = document.getElementById("dialog-name").innerText;

  document.getElementById("popup").style.display = "none";
  showConsultationPopup();
}

function showPopup(title, subtitle) {
  document.getElementById("dialog-name").innerText = title;
  document.getElementById("dialog-job").innerText = subtitle;
  document.getElementById("dialog-about").innerText = "";
  document.getElementById("contact-list").innerHTML = "";
  document.getElementById("popup").style.display = "flex";
  document.getElementById("popup-button").style.display = "none";
  document.getElementById("closePopup").addEventListener("click", function () {
    document.getElementById("popup").style.display = "none";
  });
  disableScroll();
}

// Close when clicking outside the popup content
window.addEventListener("click", function (event) {
  let popup = document.getElementById("popup");
  let consultationPopup = document.getElementById("consultation-popup");
  if (event.target === popup || event.target === consultationPopup) {
    popup.style.display = "none";
    consultationPopup.style.display = "none";
    document.getElementById("contact-list").innerHTML = "";
    enableScroll();
  }
});

// Join Us form send button handler
function join_forum_sender() {
  fetch("/join_form", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    // All validation are done on the server side
    body: JSON.stringify({
      name: document.getElementById("form-name").value,
      speciality: document.getElementById("form-job").value,
      residence: document.getElementById("form-country").value,
      phone_number: `${document.getElementById("form-country-code").value} ${document.getElementById("form-phone-number").value}`,
      email: document.getElementById("form-email").value,
      more: document.getElementById("form-other-info").value,
    }),
  })
    .then((response) => response.json())
    .then((response) => {
      if (response.success) {
        document.querySelectorAll('.form-reset-after-submit').forEach((input) => {
          input.value = '';
        });
        showPopup(
          "تم إرسال طلبك بنجاح",
          "أمهلنا بعض الوقت لمراجعة تفاصيل طلبك، توقع أن يتم التواصل معك عبر الإيميل، أو أننا سنرسل لك رسالة واتساب على نفس الرقم الذي أعطيتنا إياه، أذا تعذر كل ما سبق فسنضطر للإتصال بك. "
        );
      } else {
        showError(response.error);
      }
    })
    .catch((error) => {
      console.log(error);
      showError(error);
    });
}

function drawContact(contact, parentID) {
  const li = document.createElement("li");
  li.classList.add("contact-item");
  switch (contact.type) {
    case "email":
      li.innerHTML = `<a class="contact-link">
                                <div class="icon email-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"/>
                                    </svg>
                                </div>
                                ${contact.data}
                            </a>`;
      break;
    case "whatsapp":
      li.innerHTML = `<a class="contact-link">
                                <div class="icon whatsapp-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413z"/>
                                    </svg>
                                </div>
                                ${contact.data}
                            </a>`;
      break;
    case "facebook":
      li.innerHTML = `<a class="contact-link">
                                <div class="icon facebook-icon">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                                        <path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z"/>
                                    </svg>
                                </div>
                                ${contact.data}
                            </a>`;
      break;
    case "instagram":
      li.innerHTML = `<a class="contact-link">
                <div class="icon instagram-icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 0C8.74 0 8.333.015 7.053.072 5.775.132 4.905.333 4.14.63c-.789.306-1.459.717-2.126 1.384S.935 3.35.63 4.14C.333 4.905.131 5.775.072 7.053.012 8.333 0 8.74 0 12s.015 3.667.072 4.947c.06 1.277.261 2.148.558 2.913.306.788.717 1.459 1.384 2.126.667.666 1.336 1.079 2.126 1.384.766.296 1.636.499 2.913.558C8.333 23.988 8.74 24 12 24s3.667-.015 4.947-.072c1.277-.06 2.148-.262 2.913-.558.788-.306 1.459-.718 2.126-1.384.666-.667 1.079-1.335 1.384-2.126.296-.765.499-1.636.558-2.913.06-1.28.072-1.687.072-4.947s-.015-3.667-.072-4.947c-.06-1.277-.262-2.149-.558-2.913-.306-.789-.718-1.459-1.384-2.126C21.319 1.347 20.651.935 19.86.63c-.765-.297-1.636-.499-2.913-.558C15.667.012 15.26 0 12 0zm0 2.16c3.203 0 3.585.016 4.85.071 1.17.055 1.805.249 2.227.415.562.217.96.477 1.382.896.419.42.679.819.896 1.381.164.422.36 1.057.413 2.227.057 1.266.07 1.646.07 4.85s-.015 3.585-.074 4.85c-.061 1.17-.256 1.805-.421 2.227-.224.562-.479.96-.897 1.382-.419.419-.824.679-1.38.896-.42.164-1.065.36-2.235.413-1.274.057-1.649.07-4.859.07-3.211 0-3.586-.015-4.859-.074-1.171-.061-1.816-.256-2.236-.421-.569-.224-.96-.479-1.379-.897-.421-.419-.69-.824-.9-1.38-.165-.42-.359-1.065-.42-2.235-.045-1.26-.061-1.649-.061-4.844 0-3.196.016-3.586.061-4.861.061-1.17.255-1.814.42-2.234.21-.57.479-.96.9-1.381.419-.419.81-.689 1.379-.898.42-.166 1.051-.361 2.221-.421 1.275-.045 1.65-.06 4.859-.06l.045.03zm0 3.678c-3.405 0-6.162 2.76-6.162 6.162 0 3.405 2.76 6.162 6.162 6.162 3.405 0 6.162-2.76 6.162-6.162 0-3.405-2.76-6.162-6.162-6.162zM12 16c-2.21 0-4-1.79-4-4s1.79-4 4-4 4 1.79 4 4-1.79 4-4 4zm7.846-10.405c0 .795-.646 1.44-1.44 1.44-.795 0-1.44-.646-1.44-1.44 0-.794.646-1.439 1.44-1.439.793-.001 1.44.645 1.44 1.439z"/>
                    </svg>
                </div>
                ${contact.data}
            </a>`;
      break;
    default:
      break;
  }
  document.getElementById(parentID).appendChild(li);
}

function addDoctors(specality) {
  fetch("contributors.json")
    .then((response) => response.json())
    .then((data) => {
      data.contributors.forEach((contributor) => {
        contributor.doctors.forEach((doctor) => {
          let foundAMatch = false;

          for (let i = 0; i < specality.doctors.length; i++) {
            if (specality.doctors[i] === doctor.name) {
              foundAMatch = true;
              break;
            }
          }

          if (!foundAMatch) return;

          const div = document.createElement("div");
          div.classList.add("card");

          div.addEventListener("click", () => {
            document.getElementById("dialog-name").innerText = doctor.name;
            document.getElementById("dialog-job").innerText = doctor.job;
            document.getElementById("dialog-about").innerText = doctor.about;
            doctor.contact.forEach((contct) => {
              drawContact(contct, "contact-list");
            });
            document.getElementById("popup").style.display = "flex";
            document.getElementById("popup-button").style.display = "block";
            disableScroll();
          });

          document
            .getElementById("closePopup")
            .addEventListener("click", function () {
              document.getElementById("popup").style.display = "none";
              enableScroll();
            });

          div.innerHTML = `
                        <img src="../images/doctors/${doctor.image}" alt="">
                        <div class="card-content">
                            <h3>${doctor.name}</h3>
                            <p>${doctor.job}</p>
                        </div>
                    `;

          document.getElementById("specialties-grid").appendChild(div);
        });
      });
    })
    .catch((error) => console.error("Error fetching contributors:", error));
}

let weAreInTheSubList = false;
let bringUsBackToTheSubList = false;

function addSpeciality(specality) {
  if (weAreInTheSubList) {
    bringUsBackToTheSubList = false;
    specality.sublist.forEach((sub_speciality) => {
      document.getElementById("specialties-title").innerText = "أقسام الجراحة";
      const div = document.createElement("div");
      div.classList.add("card");
      div.style.borderRadius = "16px";
      div.style.flex = "0 1 180px";

      div.addEventListener("click", () => {
        bringUsBackToTheSubList = true;
        // Clear the grid
        document.getElementById("specialties-grid").innerHTML = "";
        addDoctors(sub_speciality);
        document.getElementById("go-back").style.display = "flex";
        document.getElementById(
          "specialties-title"
        ).innerText = `جراحة ${sub_speciality.name}`;
      });

      // Invert the colors of svg (black -> white)
      let style;
      if (sub_speciality.icon.includes("svg")) {
        style =
          "filter: invert(80%) brightness(100%); margin-top: 1rem;";
      }
      div.innerHTML = `<center><img src=\"../images/icons/${sub_speciality.icon}\" alt=\"\" style="width: 8rem; height: fit-content; ${style}"></center>
                            <div class=\"card-content\">
                                <h2>${sub_speciality.name}</h2>
                            </div>`;
      document.getElementById("specialties-grid").appendChild(div);
    });
    // Invert the boolean after we have finished doing stuff
    weAreInTheSubList = false;
  } else {
    const div = document.createElement("div");
    div.classList.add("card");
    div.style.borderRadius = "16px";
    div.style.flex = "0 1 180px";

    div.addEventListener("click", () => {
      if (specality.use_sublist) {
        weAreInTheSubList = true;
        // Clear the grid
        document.getElementById("specialties-grid").innerHTML = "";
        addSpeciality(specality);
        document.getElementById("go-back").style.display = "flex";
        return;
      }
      // Clear the grid
      document.getElementById("specialties-grid").innerHTML = "";
      addDoctors(specality);
      document.getElementById("go-back").style.display = "flex";
      document.getElementById(
        "specialties-title"
      ).innerText = `أطباء ${specality.name}`;
    });

    // Invert the colors of svg (black -> white)
    let style;
    if (specality.icon.includes("svg")) {
      style =
        "filter: invert(80%) brightness(100%); margin-top: 1rem;";
    }
    div.innerHTML = `<center><img src=\"../images/icons/${specality.icon}\" alt=\"\" style="width: 8rem; height: fit-content;  ${style}"></center>
                        <div class=\"card-content\" style="">
                            <h2>${specality.name}</h2>
                        </div>`;
    document.getElementById("specialties-grid").appendChild(div);
  }
}

function drawMainSpecialitiesMenu() {
  // Clear the grid
  document.getElementById("specialties-grid").innerHTML = "";
  fetch("specialties.json")
    .then((response) => response.json())
    .then((data) => {
      if (bringUsBackToTheSubList) {
        data.specialties.forEach((specality) => {
          if (!specality.use_sublist) return;
          weAreInTheSubList = true;
          addSpeciality(specality);
        });
      } else {
        document.getElementById("specialties-title").innerText =
          "التخصصات المتوفرة لدينا";
        document.getElementById("go-back").style.display = "none";
        data.specialties.forEach((specality) => {
          addSpeciality(specality);
        });
      }
    })
    .catch((error) => {
      console.error("Error fetching the JSON file:", error);
    });
}

drawMainSpecialitiesMenu();

fetch("contributors.json")
  .then((response) => response.json())
  .then((data) => {
    data.contributors.forEach((contributor) => {
      contributor.doctors.forEach((doctor) => {
        const div = document.createElement("div");
        div.classList.add("card");

        div.addEventListener("click", () => {
          document.getElementById("dialog-name").innerText = doctor.name;
          document.getElementById("dialog-job").innerText = doctor.job;
          document.getElementById("dialog-about").innerText = doctor.about;
          doctor.contact.forEach((contct) => {
            drawContact(contct, "contact-list");
          });
          document.getElementById("popup").style.display = "flex";
          document.getElementById("popup-button").style.display = "block";
          disableScroll();
        });

        document
          .getElementById("closePopup")
          .addEventListener("click", function () {
            document.getElementById("popup").style.display = "none";
            enableScroll();
          });

        div.innerHTML = `<img src=\"../images/doctors/${doctor.image}\" alt=\"\"><div class=\"card-content\"><h3>${doctor.name}</h3><p>${doctor.job}</p></div>`;
        document.getElementById("doctors-grid").appendChild(div);
      });
      contributor.others.forEach((other) => {
        const div = document.createElement("div");
        div.classList.add("card");

        div.addEventListener("click", () => {
          document.getElementById("dialog-name").innerText = other.name;
          document.getElementById("dialog-job").innerText = other.job;
          document.getElementById("dialog-about").innerText = other.about;
          other.contact.forEach((contct) => {
            drawContact(contct, "contact-list");
          });
          document.getElementById("popup").style.display = "flex";
          document.getElementById("popup-button").style.display = "none";
          disableScroll();
        });

        document
          .getElementById("closePopup")
          .addEventListener("click", function () {
            document.getElementById("popup").style.display = "none";
            enableScroll();
          });

        div.innerHTML = `<img src=\"../images/doctors/${other.image}\" alt=\"\"><div class=\"card-content\"><h3>${other.name}</h3><p>${other.job}</p></div>`;
        document.getElementById("others-grid").appendChild(div);
      });
    });
  })
  .catch((error) => {
    console.error("Error fetching the JSON file:", error);
  });

const doctors_grid = document.getElementById("doctors-grid");
const others_grid = document.getElementById("others-grid");
let isDoctorsGridAnimating = false;
let isOthersGridAnimating = false;

async function toggleDoctorsGrid() {
  if (isDoctorsGridAnimating) return;
  isDoctorsGridAnimating = true;

  doctors_grid.classList.toggle("show");

  if (
    document.getElementById("doctors-dropdown-button").textContent ==
    " ◀ الأطباء"
  ) {
    // Here doctors are dropped down
    document.getElementById("doctors-dropdown-button").textContent =
      " ▼ الأطباء";
    doctors_grid.style.maxHeight = "fit-content";
    isDoctorsGridAnimating = false;
  } else {
    // Here they are not
    document.getElementById("doctors-dropdown-button").textContent =
      " ◀ الأطباء";
    doctors_grid.style.maxHeight = "fit-content";
    setTimeout(
      await function () {
        doctors_grid.style.maxHeight = "0";
        isDoctorsGridAnimating = false;
      },
      500
    ); // 500 millisecond is the duration of the animation
  }
}

function toggleOthersGrid() {
  if (isOthersGridAnimating) return;
  isOthersGridAnimating = true;

  others_grid.classList.toggle("show");

  if (
    document.getElementById("others-dropdown-button").textContent ==
    " ◀ المهندسون والفنيون والصيدلانيون"
  ) {
    // Here others are dropped down
    document.getElementById("others-dropdown-button").textContent =
      " ▼ المهندسون والفنيون والصيدلانيون";
    others_grid.style.maxHeight = "fit-content";
    isOthersGridAnimating = false;
  } else {
    // Here others are not
    document.getElementById("others-dropdown-button").textContent =
      " ◀ المهندسون والفنيون والصيدلانيون";
    others_grid.style.maxHeight = "fit-content";
    setTimeout(function () {
      others_grid.style.maxHeight = "0";
      isOthersGridAnimating = false;
    }, 500); // 500 millisecond is the duration of the animation
  }
}

// Tab switching functionality
const tabButtons = document.querySelectorAll(".nav-link");
const tabContents = document.querySelectorAll(".tab-content");

// Switch to specialites tab when the book button is clicked in the main menu
document.getElementById("bookButton").addEventListener("click", () => {
  const tabId = document.getElementById("bookButton").dataset.tab;

  tabButtons.forEach((btn) => btn.classList.remove("active"));
  tabContents.forEach((content) => content.classList.remove("active"));

  document.getElementById("specialties-button").classList.add("active");
  document.getElementById(tabId).classList.add("active");
});

tabButtons.forEach((button) => {
  button.addEventListener("click", () => {
    const tabId = button.dataset.tab;

    if (tabId == "dont-switch") {
      return;
    }

    tabButtons.forEach((btn) => btn.classList.remove("active"));
    tabContents.forEach((content) => content.classList.remove("active"));

    button.classList.add("active");
    document.getElementById(tabId).classList.add("active");

    navLinks.classList.toggle("active");
  });
});

document.getElementById("join-us").addEventListener("click", () => {
  tabContents.forEach((content) => content.classList.remove("active"));

  document.getElementById("contact").classList.add("active");
});

const mobileMenuBtn = document.querySelector(".mobile-menu-btn");
const navLinks = document.querySelector(".nav-links");

mobileMenuBtn.addEventListener("click", () => {
  navLinks.classList.toggle("active");
});

// Navigation functionality
const navItems = document.querySelectorAll(".nav-link");
const sections = document.querySelectorAll("section");

navItems.forEach((item) => {
  item.addEventListener("click", (e) => {
    // Close consultation dialog when navigating throught the top bar
    document.getElementById("consultation-popup").style.display = "none";
    enableScroll();

    if (item.dataset.tab == "dont-switch") {
      return;
    }

    e.preventDefault();

    navItems.forEach((nav) => nav.classList.remove("active"));
    item.classList.add("active");

    // Close mobile menu after clicking
    if (window.innerWidth <= 768) {
      navLinks.classList.remove("active");
    }
  });
});
