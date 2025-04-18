import { navigate } from '../bundle.js';

export async function loadHomePage(edit = false, editBody = {}) {
    const response = await fetch("/api/homepage/profile");
    const data = await response.json();
    document.getElementById("content").innerHTML = getHomePageLayout();
    if (edit) {
        populateEditHomePage(data, editBody);
    } else {
        await populateHomePageContent(data);
    }
    if (await isPreviewRunning()) {
        document.getElementById("stopPreviewButton").disabled = false;
        document.getElementById("previewButton").style.display = 'none';
        document.getElementById("seePreviewButton").style.display = 'block';
        setSeePreviewButton();
    } else {
        document.getElementById("stopPreviewButton").disabled = true;
        document.getElementById("previewButton").style.display = 'block';
        document.getElementById("seePreviewButton").style.display = 'none';
        setPreviewButton();
    }
    setStopPreviewButton();
    setEditButton(edit);
    setProfileButton(edit);
    setPublishButton(edit);
}

async function isPreviewRunning() {
    try {
        const response = await fetch("http://127.0.0.1:8000", { mode: "no-cors" });
        if (response.status >= 400) {
            return false;
        } else {
            return true;
        }
    }
    catch (_) {
        return false;
    }
}

function getHomePageLayout() {
    return `
    <section id="homepage">
        <div id="homePageButtons">
            <button id="previewButton" type="submit" href="" class="button">Start Preview</button>
            <button id="seePreviewButton" type="submit" href="http://127.0.0.1:8000" target="_blank" class="button">See Preview</button>
            <button id="stopPreviewButton" type="submit" href="" class="button">Stop Preview</button>
            <button id="editProfileButton" type="submit" href="" class="button switchable">Edit Profile</button>
            <button id="seeProfileButton" type="submit" href="" class="button switchable">See Profile</button>
            <button id="publishButton" type="submit" href="" class="button switchable">Publish</button>
        </div>
        <div id="homePageContent">
    </section>
    `;
}

async function populateHomePageContent(data) {
    document.getElementById("homePageContent").innerHTML = getHomePageContent();
    document.getElementById("profilePicture").src = `./images/${data["picture"]}`;
    document.getElementById("displayName").innerHTML = data["display_name"];
    document.getElementById("profileDescription").innerHTML = data["description"];
}

function getHomePageContent() {
    return `
    <div class="homepage-content-container">
        <img id="profilePicture" src="" alt="Your Picture" class="profile-picture"/>
        <div class="text">
            <h1>I am <span id="displayName"></span></h1>
            <p id="profileDescription"></p>
            </div>
        </div>
    </div>
    `;
}

function populateEditHomePage(data, editBody = {}) {
    document.getElementById("homePageContent").innerHTML = getEditProfileContent();
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
      <h1 class="form-heading">Update Profile</h1>

      <!-- Display Name -->
      <div class="form-group">
        <label for="displayName" class="form-label">Display Name:</label>
        <input type="text" id="displayName" name="displayName" class="form-input" placeholder="Enter your display name" required>
      </div>

      <!-- Profile Picture -->
      <div class="form-group">
        <label for="profilePicture" class="form-label">Profile Picture:</label>
        <input type="file" id="profilePicture" name="profilePicture" class="form-file" accept="image/*" required>
      </div>

      <!-- Description -->
      <div class="form-group">
        <label for="profileDescription" class="form-label">Profile Description:</label>
        <textarea id="profileDescription" name="profileDescription" class="form-textarea" placeholder="Tell us about yourself" rows="4" required></textarea>
      </div>

      <!-- Submit Button -->
      <div class="form-group button-group">
        <button id="formSubmit" type="submit" class="form-button">Update Profile</button>
      </div>
    </form>
    `;
}

function setPreviewButton() {
    document.getElementById("previewButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const response = await fetch("/api/homepage/preview", {
                method: "GET",
            });
            const body = await response.json();
            if (body["success"] == true) {
                alert("Success");
                navigate("/");
            } else {
                alert("failure");
            }
        });
}

function setSeePreviewButton() {
    document.getElementById("seePreviewButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            window.open("http://127.0.0.1:8000", "_blank");
        });
}

function setStopPreviewButton() {
    document.getElementById("stopPreviewButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const response = await fetch("/api/homepage/stop", {
                method: "GET",
            });
            const body = await response.json();
            if (body["success"] == true) {
                alert("Success");
                navigate("/");
            } else {
                alert("failure");
                navigate("/error");
            }
        });
}

function setEditButton(edit) {
    if (!edit) {
        document.getElementById("editProfileButton")
            .addEventListener("click", async (e) => {
                e.target.removeAttribute('disabled');
                e.preventDefault();
                loadHomePage(true);
            });
    } else {
        document
            .getElementById("editProfileButton")
            .setAttribute('disabled', '');
    }
}

function setProfileButton(edit) {
    if (edit) {
        document
            .getElementById("seeProfileButton")
            .addEventListener("click", async (e) => {
                e.target.removeAttribute('disabled');
                e.preventDefault();
                loadHomePage(false);
            });
    } else {
        document
            .getElementById("seeProfileButton")
            .setAttribute('disabled', '');
    }
}

function setSubmitButton(data) {
    document
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const body = makeFormBody(data);
            const response = await fetch("/api/homepage/edit", {
                method: "POST",
                body: body
            });
            if (response.status >= 400) {
                navigate("/error");
            } else {
                const body = await response.json();
                if (body["success"] === true) {
                    loadHomePage(false, body);
                } else {
                    loadHomePage(true, body);
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

function setPublishButton() {
    document
        .getElementById("publishButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const response = await fetch("/api/homepage/publish");
            if (response.status >= 400) {
                navigate("/error");
            }
            else {
                const data = await response.json();
                if (data.success == true) {
                    alert("The website was successfully pushed on GitHub");
                    navigate("/");
                } else {
                    alert("Failed to push on GitHub");
                    navigate("/error");
                }
            }
        });
}
