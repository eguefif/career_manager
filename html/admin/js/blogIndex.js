import { navigate } from "../bundle.js";
export function loadBlogIndex() {
    document
        .getElementById("content")
        .innerHTML = getBlogContent();

    setNewArticleButton();
}

function getBlogContent() {
    return `
        <section id="blog-section">
            <h1>Blog</h1>
            <div id="newArticleButton" class="button new-article">
                Write an Article
            </div>
            <p>This section is under work. You can read my articles on 
            <a href="https://medium.com/@eguefif"> medium</a>.
            </p>
        </section>
        `;
}

function setNewArticleButton() {
    document
        .getElementById("newArticleButton")
        .addEventListener("click", (e) => {
            console.log("HEY");
            e.preventDefault;
            navigate("/blog/new");
        });
}
