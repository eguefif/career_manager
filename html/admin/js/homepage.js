import { navigate } from '../bundle.js';

export async function loadHomePage() {
    document.getElementById("content").innerHTML = getHomePageLayout();
    if (await isPreviewRunning()) {
        document.getElementById("stopPreviewButton").disabled = false;
        document.getElementById("stopPreviewButton").active = true;
        document.getElementById("previewButton").style.display = 'none';
        document.getElementById("seePreviewButton").style.display = 'block';
        setSeePreviewButton();
    } else {
        document.getElementById("stopPreviewButton").disabled = true;
        document.getElementById("stopPreviewButton").active = false;
        document.getElementById("previewButton").style.display = 'block';
        document.getElementById("seePreviewButton").style.display = 'none';
        setPreviewButton();
    }
    setStopPreviewButton();
    setPublishButton();
    setWriteArticleButton();
    setAddProjectButton();
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
        <div class="home-page-buttons">
            <button id="previewButton" type="submit" href="" class="button">Start Preview</button>
            <button id="seePreviewButton" type="submit" href="http://127.0.0.1:8000" target="_blank" class="button">See Preview</button>
            <button id="stopPreviewButton" type="submit" href="" class="button">Stop Preview</button>
            <button id="writeArticleButton" type="submit" href="" class="button">Write Article</button>
            <button id="addProjectButton" type="submit" href="" class="button">Add a Project</button>
            <button id="publishButton" type="submit" href="" class="button switchable">Publish</button>
        </div>
    </section>
    `;
}

function setPreviewButton() {
    document.getElementById("previewButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const response = await fetch("/api/admin/preview", {
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
            const response = await fetch("/api/admin/stop", {
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

function setPublishButton() {
    document
        .getElementById("publishButton")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const response = await fetch("/api/admin/publish");
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

function setWriteArticleButton() {
    document
        .getElementById("writeArticleButton")
        .addEventListener("click", (e) => {
            e.preventDefault();
            navigate("/blog/new");
        });
}

function setAddProjectButton() {
    document
        .getElementById("addProjectButton")
        .addEventListener("click", (e) => {
            e.preventDefault();
            navigate("/portfolio/new");
        });
}
