import { navigate } from '../bundle.js';

export function loadNewProject() {
    document
        .getElementById("content")
        .innerHTML = getNewProjectContent();
    setAddButton();
}

function getNewProjectContent() {
    return getProjectForm("Add Project", "Add project");
}

export function getProjectForm(title, submitButton, data) {
    const name = data ? data.name : "";
    const description = data ? data.description : "";
    const github = data ? data.github : "";
    const skills = data ? data.skills.join(", ") : "";
    return `
    <form id="profileForm" action="/submit" method="POST" enctype="multipart/form-data" class="form-container">
      <h1 class="form-heading">${title}</h1>

      <!-- Project Name -->
      <div class="form-group">
        <label for="projectName" class="form-label">Project Name:</label>
        <input type="text" id="projectName" name="projectName" class="form-input" placeholder="Enter your project name" value="${name}" required>
      </div>

      <!-- Profile Picture -->
      <div class="form-group">
        <label for="ProjectPicture" class="form-label">Project Picture:</label>
        <input type="file" id="projectPicture" name="projectPicture" class="form-file" accept="image/*" required>
      </div>

      <!-- Description -->
      <div class="form-group">
        <label for="projectDescription" class="form-label">Project Description:</label>
        <textarea id="projectDescription" name="projectDescription" class="form-textarea" placeholder="Describe the project" rows="4" required>${description}</textarea>
      </div>

      <!-- Github url-->
      <div class="form-group">
        <label for="github" class="form-label">Github url:</label>
        <input type="text" id="github" name="githubUrl" class="form-input" placeholder="Enter the project's github url" value="${github}" required>
      </div>

      <!-- skills -->
      <div class="form-group">
        <label for="skills" class="form-label">List of skills seperated by a comma</label>
        <input type="text" id="skills" name="skills" class="form-input" placeholder="Enter skills" value="${skills}" required>
      </div>

      <!-- Submit Button -->
      <div class="form-group button-group">
        <button id="formSubmit" type="submit" class="button">${submitButton}</button>
      </div>
    </form>
    `;
}

function setAddButton() {
    document
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const body = makeProjectBody();
            const payload = await getPayload(body);
            const response = await fetch("/api/portfolio/new", {
                method: "POST",
                headers: {
                    "Content-Type": "application/octet-stream"
                },
                body: payload,
            });

            if (response.status >= 400) {
                navigate("/error");
            } else {
                navigate("/portfolio/index");
            }
        });
}

export function makeProjectBody() {
    const name = document.getElementById("projectName").value;
    const description = document.getElementById("projectDescription").value;
    const picture = document.getElementById("projectPicture")?.files[0]?.name;
    const github = document.getElementById("github").value;
    const skills = document.getElementById("skills").value;
    return JSON.stringify({
        name: name,
        description: description,
        picture: picture ? picture : "",
        github: github,
        skills: processSkills(skills),
    });
}

function processSkills(skills) {
    const splits = skills.split(",");
    return splits.map((skill) => skill.trim());

}

export async function getPayload(body) {
    const picture = document.getElementById("projectPicture").files[0];
    if (picture) {
        const jsonBytes = new TextEncoder().encode(body);
        const sepBytes = new TextEncoder().encode("###");
        const fileBytes = await picture.arrayBuffer();

        const buffer = new Uint8Array(jsonBytes.length + sepBytes.length + fileBytes.byteLength);

        buffer.set(jsonBytes);
        buffer.set(sepBytes, jsonBytes.length);

        buffer.set(new Uint8Array(fileBytes), sepBytes.length + jsonBytes.length);
        return buffer;
    } else {
        const jsonBytes = new TextEncoder().encode(body);
        const sepBytes = new TextEncoder().encode("###");
        const buffer = new Uint8Array(jsonBytes.length + sepBytes.length);

        buffer.set(jsonBytes);
        buffer.set(sepBytes, jsonBytes.length);
        return buffer;
    }

}
