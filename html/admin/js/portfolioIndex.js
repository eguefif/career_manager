import { navigate } from "../bundle.js";

export async function loadIndex() {
    const response = await fetch("/api/portfolio/index");
    const data = await response.json();
    console.log(data);
    document.getElementById("content").innerHTML = getPortfolioContent(data);
    setAddProjectButton();
    setDeleteProjectButton();
}

function getPortfolioContent(data) {
    const projects = data.reduce((acc, project) => {
        return acc.concat(makeProject(project));
    },
        ""
    );
    return `
<h1>My Portfolio</h1>
<section id="portfolio">
<div>
<div id="addProjectButton" class="button">Add project</div>
</div>
${projects}
</section>
    `;
}

function makeProject(project) {
    return `
    <div class="project-box">
        <div id="deleteProjectButton" data-id="${project.id}" type="submit" class="delete-project-button">delete</div>
        <a href="${project.github}" class="github-link" target="_blank">
            <img src="images/github.svg" alt="GitHub" class="github-icon">
        </a>
        <img src="images/${project.picture}" alt="Project 1" class="project-img">
        <div class="project-info">
            <h2 class="project-title">${project.name}</h2>
            <p class="project-description">${project.description}</p>
            <ul class="skills-list">
            ${makeSkills(project.skills)}
            </ul>
        </div>
    </div>
    `;
}

function makeSkills(skills) {
    const skillsHtml = skills.reduce((acc, skill) => { return acc.concat(`<li>${skill}</li>`); }, "");
    return skillsHtml;
}

function setAddProjectButton() {
    document
        .getElementById("addProjectButton")
        .addEventListener("click", (e) => {
            e.preventDefault();
            navigate("/portfolio/new");
        });

}

function setDeleteProjectButton() {
    document
        .querySelectorAll(".delete-project-button")
        .forEach((btn) => btn.addEventListener("click", (e) => {
            e.preventDefault();
            const id = e.target.dataset.id;
            const url = `/api/portfolio/delete/${id}`;
            console.log("HEY: ", url);
            fetch(url);
            navigate("/portfolio/index");
        }));

}
