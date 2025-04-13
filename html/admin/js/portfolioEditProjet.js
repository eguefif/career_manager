import { navigate } from '../bundle.js';
import { makeProjectBody, getPayload, getProjectForm } from './portfolioNewProject.js';

export function editProject(id) {
    const data = fetchProject(id);
    if (data) {
        document
            .getElementById("content")
            .innerHTML = getEditProjectView(id);
        setUpdateButton();
    } else {
        navigate("/error");
    }
}

function getEditProjectView(data) {
    return getProjectForm("Edit Project", "Update Project", data);
}

async function fetchProject(id) {
    const response = await fetch(`/api/portfolio/show/${id}`);
    if (response.status <= 400) {
        return await response.json();
    }
}

function setUpdateButton() {
    document
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const body = makeProjectBody();
            const payload = await getPayload(body);
            console.log(payload);
            const response = await fetch("/api/portfolio/update", {
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
