document.addEventListener("DOMContentLoaded", async function() {
    window.addEventListener("popstate", handleRoute);

    initRouter();
    await handleRoute()
});

function initRouter() {
    const links = document.querySelectorAll(".nav a");

    links.forEach(link => {
        link.addEventListener("click", async (e) => {
            e.preventDefault();
            const url = new URL(e.target.href)
            navigate(url.pathname);
            await handleRoute();
        });
    });
}

function navigate(route) {
    const routes = [
        { title: "Portfolio", path: "portfolio" },
        { title: "Blog", path: "blog" },
        { title: "Article", path: "articles" }
    ];

    const routeData = routes.find(data =>  route.includes(data.path) );
    if (routeData) {
        history.pushState({}, routeData.title, route);
    } else {
        history.pushState({}, "Home", "/");
    }
}

async function handleRoute() {
    const route = window.location.pathname;
    if (route == "/portfolio") {
        loadPortfolio();
    } else if (route == "/blog") {
        loadBlog();
    } else if (route.includes("articles")) {
        await loadArticle(route);
    } else {
        loadHome();
    }
}

function loadPortfolio() {
    const link = document.getElementById("portfolio-link");
    document.getElementById("content").innerHTML = getPortfolioContent();
}

function loadHome() {
    const link = document.getElementById("home-link");
    document.getElementById("content").innerHTML = getHomeContent();
}

function loadBlog() {
    document.getElementById("content").innerHTML = getBlogContent();
    const links = document.querySelectorAll(".article-box a");

    links.forEach(link => {
        link.addEventListener("click", async (e) => {
            e.preventDefault();
            const url = new URL(e.currentTarget.href)
            navigate(url.pathname);
            await handleRoute();
        });
    });
}

function getBlogContent() {
    return `
        <section id="blog-section">
            <h1>Blog</h1>
            <p>This section is under work. You can read my articles on 
            <a href="https://medium.com/@eguefif"> medium</a>.
            </p>
        </section>
        `
}

function getHomeContent() {
    return `
    {{use("home_page.html")}}
    `
}

function getPortfolioContent() {
    return `
    {{use("projects.html")}}
    `
}

function getBlogContent() {
    return `
    {{use("blog.html")}}
    `
}

async function loadArticle(route) {
    let id = extractId(route);
    document.getElementById("content").innerHTML = await getArticleContent(id);
}

function extractId(route) {
    let splits = route.split("/");
    return splits[splits.length - 1];
}

async function getArticleContent(id) {
    let url = `../articles/${id}.html`;

    let response = await fetch(url);
    if (response.status == 200) {
        const body = await response.text();
        return body;
    } else {
        return `<center><h1>Article not found</center></h1>`
    }
}
