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
    <section id="homepage">
    <h1>I am Emmanuel Guefif</h1>
    <div class="homepage-container">
        <img src="./images/emmanuel.jpeg" alt="Your Picture" class="homepage-img">
        <div class="homepage-text">
            <p>
                Lifelong learner, I made my first program when I was sixteen. It was a GCD calculator implementing an algorithm I had learned at school. In the first part of my adult life, I studied sociology and then became a teacher to share my passion for learning and nurture my student's curiosity. After meeting one of my student's father, I realized that talking about computers made me feel very good, and I decided to turn what was a passion into a profession. I now work as a full stack developper and learn everything I can about architecture.
        </p>
        </div>
    </div>
</section>

    `
}

function getPortfolioContent() {
    return `
    <h1>My Portfolio</h1>
    <section id="portfolio">
    
        <div class="project-box">
        <a href="https://github.com/PelletierM/miniRT" class="github-link" target="_blank">
            <img src="./images/github.svg" alt="GitHub" class="github-icon" />
        </a>

        <img src="images/spheres.png" alt="Project 1" class="project-img">
            <div class="project-info">
            <h2 class="project-title">Ray tracer</h2>
            <p class="project-description">This 42 project is all about Ray Tracing. I worked mainly on implementing different figures: sphere, plane, cylinder, and triangle. We optimized rendering by implementing a sample accumulator. We also took advantage of multithreading to render rays in batches.</p>
            <ul class="skills-list">
            
                    <li>language C</li>
            
                    <li>multithreading</li>
            
                    <li>Ray Tracing</li>
            
            </ul>
            </div>
        </div>
    
        <div class="project-box">
        <a href="https://github.com/demarque/marc-record-ex" class="github-link" target="_blank">
            <img src="./images/github.svg" alt="GitHub" class="github-icon" />
        </a>

        <img src="images/marc21.jpg" alt="Project 1" class="project-img">
            <div class="project-info">
            <h2 class="project-title">Marc-record-ex</h2>
            <p class="project-description">I made this project for my work at Demarque. It binds a Marc record library written in Rust with an Elixir package. When I used this project in the parser, I had to optimize memory usage to avoid upgrading our pod. I use streaming techniques and find a way to make it work between the Rust bindings and Elixir code.</p>
            <ul class="skills-list">
            
                    <li>Rust</li>
            
                    <li>Rustler</li>
            
                    <li>Elixir</li>
            
            </ul>
            </div>
        </div>
    
        <div class="project-box">
        <a href="https://github.com/eguefif/monkey_interpreter" class="github-link" target="_blank">
            <img src="./images/github.svg" alt="GitHub" class="github-icon" />
        </a>

        <img src="images/monkey.gif" alt="Project 1" class="project-img">
            <div class="project-info">
            <h2 class="project-title">Monkey Interpreter</h2>
            <p class="project-description">This project was made when I was reading the book 'Make a Monkey interpreter in Go. ' I did it in Rust. I've learned a lot about parsing and recursion. It was also an opportunity to learn about Rust smart pointers.</p>
            <ul class="skills-list">
            
                    <li>Rust</li>
            
                    <li>parsing</li>
            
            </ul>
            </div>
        </div>
    
        <div class="project-box">
        <a href="https://github.com/eguefif/game_boy_emulator" class="github-link" target="_blank">
            <img src="./images/github.svg" alt="GitHub" class="github-icon" />
        </a>

        <img src="images/tetris.jpg" alt="Project 1" class="project-img">
            <div class="project-info">
            <h2 class="project-title">GameBoy Emulator</h2>
            <p class="project-description">This Game Boy emulator can run Tetris. This was not an easy project. There is no official documentation, but you have to gather a lot of resources yourself. It was an opportunity to learn by looking at other people's code and understanding the logic.</p>
            <ul class="skills-list">
            
                    <li>Rust</li>
            
                    <li>CPU architecture</li>
            
                    <li>System Interrupt</li>
            
                    <li>Low-level Rendering</li>
            
            </ul>
            </div>
        </div>
    
    </section>

    `
}
