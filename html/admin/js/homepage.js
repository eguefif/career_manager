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
    <h1> Edit Profile</h1>
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
