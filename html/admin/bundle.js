import { loadHomePage } from "./js/homepage.js";
import { loadPortfolioPage } from "./js/portfolio.js";
import { loadErrorPage } from "./js/errorpage.js";
import { loadBlog } from "./js/blog.js";

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
        { title: "Blog", path: "/blog/index" },
        { title: "Blog new", path: "/blog/new" },
        { title: "Blog edit", path: "/blog/edit" },
        { title: "Error", path: "/error" },
    ];

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
    const secondLevelRoute = extractRoute(route, 1);
    switch (firstLevelRoute) {
        case "portfolio":
            switch (secondLevelRoute) {
                case "index":
                    loadPortfolioPage();
                    break;
                case "new":
                    loadPortfolioPage("new");
                    break;
                case "edit":
                    const id = extractRoute(route, 2);
                    loadPortfolioPage("edit", id);
                    break;
                default:
                    loadBlog();
                    break;
            }
            break;
        case "blog":
            switch (secondLevelRoute) {
                case "index":
                    await loadBlog("index");
                    break;
                case "new":
                    await loadBlog("new");
                    break;
                case "edit":
                    const id = extractRoute(route, 2);
                    loadBlog("edit", id);
                    break;
                default:
                    await loadBlog("index");
                    break;
            }
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

