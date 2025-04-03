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

function getPortfolioContent() {
        return `
<section id="portfolio">
    <h1>My Portfolio</h1>
    
    <div class="project-box">
    <a href="https://github.com/yourusername/yourproject" class="github-link" target="_blank">
        <img src="https://upload.wikimedia.org/wikipedia/commons/9/91/Octicons-mark-github.svg" alt="GitHub" class="github-icon" />
    </a>

        <img src="project1.jpeg" alt="Project 1" class="project-img">
        <div class="project-info">
            <h2 class="project-title">Project Title 1</h2>
            <p class="project-description">This is a brief description of the project, showcasing its key features and functionality.</p>
            <ul class="skills-list">
                <li>HTML</li>
                <li>CSS</li>
                <li>JavaScript</li>
            </ul>
        </div>
    </div>
</section>
        `
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
    <section id="who-i-am-section">
        <h1>I am {Emmanuel Guefif}</h1>
        <div class="who-i-am-container">
            <img src="{emmanuel.jpeg}" alt="Your Picture" class="who-i-am-img">
            <div class="who-i-am-text">
                <p>
                    {Lifelong learner, I made my first program when I was sixteen. It was a GCD calculator implementing an algorithm I had learned at school. In the first part of my adult life, I studied sociology and then became a teacher to share my passion for learning and nurture my student's curiosity. After meeting one of my student's father, I realized that talking about computers made me feel very good, and I decided to turn what was a passion into a profession. I now work as a full stack developper and learn everything I can about architecture.}
            </p>
            </div>
        </div>
    </section>
    `
}
