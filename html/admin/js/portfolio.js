import { loadIndex } from './portfolioIndex.js';
import { loadNewProject } from './portfolioNewProject.js';
import { loadEditProject } from './portfolioEditProject.js';

export async function loadPortfolioPage(mode = "index", id) {
    switch (mode) {
        case "index":
            await loadIndex();
            break;
        case "edit":
            await loadEditProject(id);
            break;
        case "new":
            loadNewProject();
            break;
    }
}



