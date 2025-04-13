import { navigate } from '../bundle.js';
import { makeProjectBody, getPayload, getProjectForm } from './portfolioNewProject.js';

export async function loadEditProject(id) {
    const data = await fetchProject(id);
    if (data) {
        document
            .getElementById("content")
            .innerHTML = getEditProjectView(data);
        setUpdateButton(data.id);
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

function setUpdateButton(id) {
    document
        .getElementById("formSubmit")
        .addEventListener("click", async (e) => {
            e.preventDefault();
            const body = makeProjectBody();
            const payload = await getPayload(body);
            const url = `/api/portfolio/update/${id}`;
            const response = await fetch(url, {
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
