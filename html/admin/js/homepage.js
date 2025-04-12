export async function loadHomePage(edit = false) {
    const response = await fetch("/api/homepage/profile");
    const data = await response.json();
    document.getElementById("content").innerHTML = getHomePageLayout();
    if (edit) {
        populateEditHomePage();
    } else {
        await populateHomePageContent(data);
    }
    setBuildButton();
    setEditButton(edit);
    setProfileButton(edit);
}

function getHomePageLayout() {
    return `
    <section id="homepage">
        <div id="homePageButtons">
            <button id="buildButton" type="submit" href="" class="button">Build website</button>
            <button id="editProfileButton" type="submit" href="" class="button switchable">Edit Profile</button>
            <button id="seeProfileButton" type="submit" href="" class="button switchable">See Profile</button>
        </div>
        <div id="homePageContent">
    </section>
    `;
}

async function populateHomePageContent(data) {
    document.getElementById("homePageContent").innerHTML = getHomePageContent();
    document.getElementById("displayName").innerHTML = data["displayName"];
    document.getElementById("profilePicture").src = data["picture"];
    document.getElementById("profileDescription").innerHTML = data["description"];
}

function getHomePageContent() {
    return `
    <div class="homepage-content-container">
        <img id="profilePicture" src="" alt="Your Picture" class="profile-picture"/>
        <div clas="text">
            <h1>I am <span id="displayName"></span></h1>
            <p id="profileDescription"></p>
            </div>
        </div>
    </div>
    `;
}

function populateEditHomePage() {
    document.getElementById("homePageContent").innerHTML = getEditProfileContent();
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
        <button type="submit" class="form-button">Update Profile</button>
      </div>
    </form>
    `;
}

function setBuildButton() {
    document.getElementById("buildButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const response = await fetch("/api/homepage/build", {
                method: "POST",
                body: "build",
            });
            const body = await response.json();
            if (body["result"] == "success") {
                alert("Success");
            } else {
                alert("failure");
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
