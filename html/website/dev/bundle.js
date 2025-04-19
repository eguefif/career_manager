document.addEventListener("DOMContentLoaded", function() {
    window.addEventListener("popstate", handleRoute);

    initRouter();
    handleRoute()
});

function initRouter() {
    const links = document.querySelectorAll(".nav a");

    links.forEach(link => {
        link.addEventListener("click", (e) => {
            e.preventDefault();
            const url = new URL(e.target.href)
            navigate(url.pathname);
            handleRoute();
        });
    });
}

function navigate(route) {
    const routes = [
        { title: "Home", path: "/" },
        { title: "Portfolio", path: "/portfolio" },
        { title: "Blog", path: "/blog" }
    ];

    const routeData = routes.find(data => data.path == route);
    if (routeData) {
        history.pushState({}, routeData.title, routeData.path);
    } else {
        history.pushState({}, "Home", "/");
    }
}

function handleRoute() {
    const route = window.location.pathname;
    if (route == "/portfolio") {
        loadPortfolio();
    } else if (route == "/blog") {
        loadBlog();
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
