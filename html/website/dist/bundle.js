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
    <section id="who-i-am-section">
    <h1>I am Emmanuel Guefif</h1>
    <div class="who-i-am-container">
        <img src="./images/emmanuel.jpeg" alt="Your Picture" class="who-i-am-img">
        <div class="who-i-am-text">
            <p>
                Lifelong learner, I made my first program when I was sixteen. It was a GCD calculator implementing an algorithm I had learned at school. In the first part of my adult life, I studied sociology and became a teacher to share my passion for learning and nurture my students' curiosity. After meeting one of my students' fathers, I realized that talking about computers made me feel very good, and I decided to turn what was a passion into a profession. I now work as a full-stack developer and learn everything I can about architecture.
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
            <p class="project-description">This 42 project is all about Ray Tracing. I worked especially on the implementation of different figure: sphere, place, cylinder, triangle. We optimized rendering by implementing a sample accumulator. We also take advantage of multithreading to render ray by batch.</p>
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
            <p class="project-description">I made this project for my work at Demarque. This project bind a marc record library with an elixir package. When I used this project in the parser, I had to optimize memory usage.</p>
            <ul class="skills-list">
            
                    <li>Rust</li>
            
                    <li>Rustler</li>
            
                    <li>memory</li>
            
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
            <p class="project-description">This project was made when I was reading the book, 'Make a Monkey interpreter in Go. I did in Rust. I've learned a lot about parsing and recursion.</p>
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
            <p class="project-description">This GameBoy emulator is able make Tetris works. This was not an easy project. There is no official documentation but there is a lot of resource you have to gather yourself. It was the occasion to learn by looking at other people code and understand the logic.</p>
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
