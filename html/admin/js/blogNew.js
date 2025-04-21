import { navigate } from "../bundle.js";

export function loadNewArticle() {
    document
        .getElementById("content")
        .innerHTML = getNewArticleForm();
    setSaveArticleButton();
}

export function getNewArticleForm(article) {
    const title = article ? article.title : "Title";
    const content = article ? article.content : "Write the content here";
    return `
    <div class="article-form">
        <form id="articleForm" action="/submit" method="POST" enctype="multipart/form-data" class="form-container">
          <h1 class="form-heading">Write Article</h1>
          <!-- Submit Button -->
          <div class="form-group button-group">
            <button id="formSubmit" type="submit" class="button">Save Article</button>
          </div>

          <div class="form-group">
            <input type="text" id="title" name="title" class="form-input" value="${title}" required>
          </div>

          <div class="form-group">
            <textarea id="content-form" onfocus="this.value = ''" name="content" class="form-textarea" required>${content}</textarea>
          </div>

        </form>
    </div>
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
                await response.json();
                navigate("/blog/index");
            }
        });
}

export function makeFormBody() {
    return {
        title: document.getElementById("title").value,
        content: document.getElementById("content-form").value,
    };
}
