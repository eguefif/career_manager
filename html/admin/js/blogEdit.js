import { navigate } from "../bundle.js";
import { getNewArticleForm } from "./blogNew.js";

export async function loadEditArticle(id) {
    const article = await fetchArticle(id);
    document
        .getElementById("content")
        .innerHTML = getEditForm(article);

    setUpdateButton();
}

function getEditForm(article) {
    return `
    <center><h1>Edit Article</h1>
    ${getNewArticleForm(article)}
    `;
}

function setUpdateButton() {
    document
        .querySelectorAll(".edit-article-button")
        .forEach((btn) => btn.addEventListener("click", async (e) => {
            e.preventDefault();
            const id = e.target.dataset.id;
            const url = `/api/blog/edit/${id}`;
            const response = await fetch(url);
            if (response.status < 400) {
                navigate("/blog/index");
            } else {
                navigate("/error");
            }
        }));
}

async function fetchArticle(id) {
    const url = `/api/article/show/${id}`;
    const response = await fetch(url);
    if (response.status == 200) {
        const body = await response.json();
        return body;
    } else {
        navigate("/error/");
    }

}
