import { navigate } from "../bundle.js";
import { makeFormBody, getNewArticleForm } from "./blogNew.js";

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
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const id = e.target.dataset.id;
            const body = JSON.stringify(makeFormBody());
            console.log(body);
            const url = `/api/blog/update/${id}`;
            const response = await fetch(url, {
                method: "POST",
                body: body
            });
            if (response.status < 400) {
                navigate("/blog/index");
            } else {
                navigate("/error");
            }
        });
}

async function fetchArticle(id) {
    const url = `/api/blog/show/${id}`;
    const response = await fetch(url);
    if (response.status == 200) {
        const body = await response.json();
        return body;
    } else {
        navigate("/error/");
    }

}
