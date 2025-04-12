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
        { title: "Portfolio", path: "/portfolio" },
        { title: "Blog", path: "/blog" },
        { title: "Error", path: "/error" },
    ];

    const routeData = routes.find((data) => data.path == route);
    if (routeData) {
        history.pushState({}, routeData.title, routeData.path);
    } else {
        history.pushState({}, "Home", "/");
    }
    handleRoute();
}

async function handleRoute() {
    const route = window.location.pathname;
    if (route == "/portfolio") {
        loadPortfolio();
    } else if (route == "/blog") {
        loadBlog();
    } else if (route == "/error") {
        loadErrorPage();
    } else {
        await loadHomePage();
    }
}

function loadPortfolio() {
    document.getElementById("content").innerHTML = getPortfolioContent();
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
