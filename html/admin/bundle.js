import { loadHomePage } from "./js/homepage.js";
import { loadPortfolioPage } from "./js/portfolio.js";
import { loadErrorPage } from "./js/errorpage.js";

document.addEventListener("DOMContentLoaded", function () {
    window.addEventListener("popstate", handleRoute);

    initRouter();
    handleRoute();
});

function initRouter() {
    const links = document.querySelectorAll(".nav a");

    links.forEach((link) => {
        link.addEventListener("click", (e) => {
            e.preventDefault();
            const url = new URL(e.target.href);
            navigate(url.pathname);
        });
    });
}

export function navigate(route) {
    const routes = [
        { title: "Home", path: "/" },
        { title: "Portfolio index", path: "/portfolio/index" },
        { title: "Portfolio new", path: "/portfolio/new" },
        { title: "Portfolio list", path: "/portfolio/edit" },
        { title: "Blog", path: "/blog" },
        { title: "Error", path: "/error" },
    ];

    console.log(route);
    console.log(route.includes("/portfolio/index"));
    const routeData = routes.find((data) => route.includes(data.path));
    if (routeData) {
        history.pushState({}, routeData.title, route);
    } else {
        history.pushState({}, "Home", "/");
    }
    handleRoute();
}

async function handleRoute() {
    const route = window.location.pathname;
    const firstLevelRoute = extractRoute(route, 0);
    console.log(route);
    switch (firstLevelRoute) {
        case "portfolio":
            const secondLevelRoute = extractRoute(route, 1);
            console.log(secondLevelRoute);
            switch (secondLevelRoute) {
                case "index":
                    loadPortfolioPage();
                    break;
                case "new":
                    loadPortfolioPage("new");
                    break;
                case "edit":
                    const id = extractRoute(route, 2);
                    console.log(id);
                    loadPortfolioPage("edit", id);
                    break;
            }
            break;
        case "blog":
            loadBlog();
            break;
        case "error":
            loadErrorPage();
            break;
        default:
            await loadHomePage();
            break;
    }
}

export function extractRoute(uri, level) {
    if (uri[0] == "/") {
        uri = uri.substring(1);
    }
    const splits = uri.split("/");
    return splits[level];
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
        `;
}
