import { navigate } from "../bundle.js";

export async function loadBlogIndex() {
    const articles = await fetchArticles();
    document
        .getElementById("content")
        .innerHTML = getBlogContent(articles);

    setNewArticleButton();
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
        ${getTitleHtml(article.title, article.date)}
        </div>
        `;
        }, "");
    }
    return `<center><h2>No articles</h2></center>`;
}

function getTitleHtml(title, date) {
    return `
    <h3 class="index-title-article">${title}</h3>
    <span class="index-article-date">${date}</span>
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
