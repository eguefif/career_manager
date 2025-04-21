import { navigate } from '../bundle.js';

export async function loadProfilePage(edit = false, editBody = {}) {
    const response = await fetch("/api/profile/profile");
    const data = await response.json();
    document.getElementById("content").innerHTML = getProfilePageLayout(edit);
    if (edit) {
        populateEditProfilePage(data, editBody);
    } else {
        await populateProfilePageContent(data);
    }
    setEditButton(edit);
}

function getProfilePageLayout(edit) {
    let button = "";
    if (!edit) {
        button = getEditButton();
    }


    return `
    <section id="profile">
        ${button}
        <div id="ProfilePageContent">
        </div>
    </section>
    `;
}

function getEditButton() {
    return `<div id="ProfilePageButtons">
            <button id="editProfileButton" type="submit" href="" class="button">Edit Profile</button>
        </div>
        `;
}

async function populateProfilePageContent(data) {
    document.getElementById("ProfilePageContent").innerHTML = getProfilePageContent();
    document.getElementById("profilePicture").src = `./images/${data["picture"]}`;
    document.getElementById("displayName").innerHTML = data["display_name"];
    document.getElementById("profileDescription").innerHTML = data["description"];
}

function getProfilePageContent() {
    return `
    <div class="profile-container">
        <img id="profilePicture" src="" alt="Your Picture" class="profile-img"/>
        <div class="text">
            <h1>I am <span id="displayName"></span></h1>
            <p id="profileDescription"></p>
            </div>
        </div>
    </div>
    `;
}

function populateEditProfilePage(data, editBody = {}) {
    document.getElementById("ProfilePageContent").innerHTML = getEditProfileContent();
    if (editBody["displayName"]) {
        document.getElementById("displayName").value = editBody["display_name"];
        document.getElementById("displayName").style.border = "2px solid red";
    } else {
        document.getElementById("displayName").value = data["display_name"];
    }

    if (editBody["description"]) {
        document.getElementById("profileDescription").value = editBody["description"];
        document.getElementById("profileDescription").style.border = "2px solid red";
    } else {
        document.getElementById("profileDescription").value = data["description"];
    }
    setSubmitButton(data);
}

function getEditProfileContent() {
    return `
    <form id="profileForm" action="/submit" method="POST" enctype="multipart/form-data" class="form-container">
      <h1>Update Profile</h1>

      <!-- Display Name -->
      <div class="form-group">
        <label for="displayName" class="form-label"><h4>Display Name</h4></label>
        <input type="text" id="displayName" name="displayName" class="form-input" placeholder="Enter your display name" required>
      </div>

      <!-- Profile Picture -->
      <div class="form-group">
        <label for="profilePicture" class="form-label"><h4>Profile Picture</h4></label>
        <input type="file" id="profilePicture" name="profilePicture" class="form-file" accept="image/*" required>
      </div>

      <!-- Description -->
      <div class="form-group">
        <label for="profileDescription" class="form-label"><h4>Profile Description</h4></label>
        <textarea id="profileDescription" name="profileDescription" class="form-textarea" placeholder="Tell us about yourself" rows="4" required></textarea>
      </div>

      <!-- Submit Button -->
      <div class="form-group button-group">
        <button id="formSubmit" type="submit" class="button">Update Profile</button>
      </div>
    </form>
    `;
}


function setEditButton(edit) {
    if (!edit) {
        document.getElementById("editProfileButton")
            .addEventListener("click", async (e) => {
                e.target.removeAttribute('disabled');
                e.preventDefault();
                loadProfilePage(true);
            });
    }
}

function setSubmitButton(data) {
    document
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const body = makeFormBody(data);
            const response = await fetch("/api/profile/edit", {
                method: "POST",
                body: body
            });
            if (response.status >= 400) {
                navigate("/error");
            } else {
                const body = await response.json();
                if (body["success"] === true) {
                    loadProfilePage(false, body);
                } else {
                    loadProfilePage(true, body);
                }
            }
        });
}

function makeFormBody(data) {
    const displayName = document.getElementById("displayName").value;
    const description = document.getElementById("profileDescription").value;
    let picture = document.getElementById("profilePicture").value;
    if (!picture) {
        picture = data["picture"];
    }
    return JSON.stringify({
        display_name: displayName,
        description: description,
        picture: picture,
        id: data["id"]
    });
}
