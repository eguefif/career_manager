import { navigate } from "../bundle.js";

export async function loadIndex() {
    const response = await fetch("/api/portfolio/index");
    const data = await response.json();
    document.getElementById("content").innerHTML = getPortfolioContent(data);
    setAddProjectButton();
    setDeleteProjectButton();
    setEditProjectButton();
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
        <div id="deleteProjectButton" data-id="${project.id}" type="submit" class="delete-project-button" aria-label="Delete">&times;</div>
        <div class="edit-project-button" data-id="${project.id}" aria-label="Edit">
            <div class="svg-edit">
                ${getEditSvg()}
            </div>
        </div>
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
        .forEach((btn) => btn.addEventListener("click", async (e) => {
            e.preventDefault();
            const id = e.target.dataset.id;
            const url = `/api/portfolio/delete/${id}`;
            const response = await fetch(url);
            if (response.status < 400) {
                navigate("/portfolio/index");
            } else {
                navigate("/error");
            }
        }));

}

function setEditProjectButton() {
    document
        .querySelectorAll(".edit-project-button")
        .forEach((btn) => btn.addEventListener("click", async (e) => {
            e.preventDefault();
            const id = e.currentTarget.dataset.id;
            const uri = `/portfolio/edit/${id}`;
            navigate(uri);
        }));

}

export function getEditSvg() {
    return `<?xml version="1.0" encoding="utf-8"?><!-- Uploaded to: SVG Repo, www.svgrepo.com, Generator: SVG Repo Mixer Tools -->
<svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
<path fill-rule="evenodd" clip-rule="evenodd" d="M20.8477 1.87868C19.6761 0.707109 17.7766 0.707105 16.605 1.87868L2.44744 16.0363C2.02864 16.4551 1.74317 16.9885 1.62702 17.5692L1.03995 20.5046C0.760062 21.904 1.9939 23.1379 3.39334 22.858L6.32868 22.2709C6.90945 22.1548 7.44285 21.8693 7.86165 21.4505L22.0192 7.29289C23.1908 6.12132 23.1908 4.22183 22.0192 3.05025L20.8477 1.87868ZM18.0192 3.29289C18.4098 2.90237 19.0429 2.90237 19.4335 3.29289L20.605 4.46447C20.9956 4.85499 20.9956 5.48815 20.605 5.87868L17.9334 8.55027L15.3477 5.96448L18.0192 3.29289ZM13.9334 7.3787L3.86165 17.4505C3.72205 17.5901 3.6269 17.7679 3.58818 17.9615L3.00111 20.8968L5.93645 20.3097C6.13004 20.271 6.30784 20.1759 6.44744 20.0363L16.5192 9.96448L13.9334 7.3787Z" fill="#2563eb"/>
</svg>`;
}
