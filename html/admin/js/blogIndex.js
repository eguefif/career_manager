import { navigate } from "../bundle.js";
import { getEditSvg } from "./portfolioIndex.js";

export async function loadBlogIndex() {
    const articles = await fetchArticles();
    console.log(articles);
    document
        .getElementById("content")
        .innerHTML = getBlogContent(articles);

    setNewArticleButton();
    setDeleteButton();
    setEditButton();
}

function getBlogContent(articles) {
    const articlesHtml = getArticleTitles(articles);
    return `
        <section id="blog-section">
            <h1>Blog</h1>
            <div id="newArticleButton" class="button new-article">
                Write an Article
            </div>
            ${articlesHtml}
        </section>
        `;
}

function getArticleTitles(articles) {
    if (articles && articles.length > 0) {
        return articles.reduce((acc, article) => {
            return `${acc}
    <div class="article-box">
        <div data-id="${article.id}" type="submit" class="delete-article-button" aria-label="Delete">
            &times;
        </div>
        <div class="edit-article-button" data-id="${article.id}" aria-label="Edit">
            <div class="svg-edit">
                ${getEditSvg()}
            </div>
        </div>
        ${getTitleHtml(article.title, article.date)}
    </div>
        `;
        }, "");
    }
    return `<center><h2>No articles</h2></center>`;
}

function getTitleHtml(title, date) {
    return `
    <div class="article-body">
        <h3>${title}</h3>
        <div class="date">${date}</div>
    </div>
    `;
}

function setNewArticleButton() {
    document
        .getElementById("newArticleButton")
        .addEventListener("click", (e) => {
            e.preventDefault();
            navigate("/blog/new");
        });
}

async function fetchArticles() {
    const response = await fetch("/api/blog/index");
    if (response.status == 200) {
        const body = await response.json();
        return body;
    } else {
        navigate("/error");
    }
}

function setDeleteButton() {
    document
        .querySelectorAll(".delete-article-button")
        .forEach((btn) => btn.addEventListener("click", async (e) => {
            e.preventDefault();
            const id = e.target.dataset.id;
            const url = `/api/blog/delete/${id}`;
            const response = await fetch(url);
            if (response.status < 400) {
                navigate("/blog/index");
            } else {
                navigate("/error");
            }
        }));
}

function setEditButton() {
    document
        .querySelectorAll(".edit-article-button")
        .forEach((btn) => btn.addEventListener("click", async (e) => {
            e.preventDefault();
            const id = e.currentTarget.dataset.id;
            const url = `/blog/edit/${id}`;
            navigate(url);
        }));
}
