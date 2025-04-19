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
    console.log(route.includes("/blog"));
    const routes = [
        { title: "Portfolio", path: "portfolio" },
        { title: "Blog", path: "blog" },
        { title: "Article", path: "articles" }
    ];

    const routeData = routes.find(data =>  route.includes(data.path) );
    if (routeData) {
        history.pushState({}, routeData.title, routeData.path);
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
    console.log(links);

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
    console.log("in load route: ", route);
    let id = extractId(route);
    document.getElementById("content").innerHTML = await getArticleContent(id);
}

function extractId(route) {
    let splits = route.split("/");
    console.log("in extract id: ", splits);
    console.log("in extract id: ", splits[1]);
    return splits[1];
}

async function getArticleContent(id) {
    let url = `../articles/${id}.html`;

    console.log("in get content: ", url);
    let response = fetch(url);
    if (response.status == 200) {
        const body = await response.body();
        console.log("in get content: ", body);
        return body;
    } else {
        console.log("ERROR Fetching article");
        return `<center><h1>Article not found</center></h1>`
    }
}
