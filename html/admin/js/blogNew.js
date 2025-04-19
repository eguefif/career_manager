import { navigate } from "../bundle.js";

export function loadNewArticle() {
    document
        .getElementById("content")
        .innerHTML = getNewArticleForm();
    setSaveArticleButton();
}

function getNewArticleForm() {
    return `
    <form id="articleForm" action="/submit" method="POST" enctype="multipart/form-data" class="form-container">
      <h1 class="form-heading">Update Profile</h1>
      <!-- Submit Button -->
      <div class="form-group button-group">
        <button id="formSubmit" type="submit" class="form-button">Save Article</button>
      </div>

      <div class="form-group">
        <input type="text" id="title" name="title" class="form-input" required>
      </div>

      <div class="form-group">
        <textarea id="content-form" name="content" class="form-textarea" rows="40" cols="80" required></textarea>
      </div>

    </form>
    `;
}

function setSaveArticleButton() {
    document
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const body = JSON.stringify(makeFormBody());
            const response = await fetch("/api/blog/new", {
                method: "POST",
                body: body,
            });
            if (response.status >= 400) {
                navigate("/error");
            } else {
                const body = await response.json();
                console.log(body);
                navigate("/blog/index");
            }
        });
}

function makeFormBody() {
    return {
        title: document.getElementById("title").value,
        content: document.getElementById("content-form").value,
    };
}
