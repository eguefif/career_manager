export async function loadHomePage() {
    document.getElementById("content").innerHTML = getHomePageContent();
    await populateHomePageContent();
    setBuildButton();
    setEditButton();
}

async function populateHomePageContent() {
    const response = await fetch("/api/homepage/profile");
    const data = await response.json();
    document.getElementById("content").innerHTML = getHomePageContent();
    document.getElementById("displayName").innerHTML = data["displayName"];
    document.getElementById("profilePicture").src = data["picture"];
    document.getElementById("profileDescription").innerHTML = data["description"];
}

function getHomePageContent() {
    return `
    <section id="who-i-am-section">
        <button id="buildButton" type="submit" href="" class="button">Build website</button>
        <button id="editProfileButton" type="submit" href="" class="button">Edit Profile</button>
        <h1>I am <span id="displayName"></span></h1>
        <div class="who-i-am-container">
            <img id="profilePicture" src="" alt="Your Picture" class="who-i-am-img">
            <div class="who-i-am-text">
                <p id="profileDescription">
            </p>
            </div>
        </div>
    </section>
    `;
}

function loadEditProfileContent() {
    document.getElementById("content").innerHTML = getEditProfileContent();
    const buildButton = document.getElementById("seeProfileButton");
    buildButton.addEventListener("click", async (_) => {
        loadHomePage();
    });
}

function getEditProfileContent() {
    return `
    <section id="who-i-am-section">
        <button id="seeProfileButton" type="submit" href="" class="button">See Profile</button>
        <h1> Edit Profile</h1>
    </section>
    `;
}

function setBuildButton() {
    const buildButton = document.getElementById("buildButton");
    buildButton.addEventListener("click", async (e) => {
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

function setEditButton() {
    const buildButton = document.getElementById("editProfileButton");
    buildButton.addEventListener("click", async (e) => {
        e.preventDefault();
        loadEditProfileContent();
    });
}
