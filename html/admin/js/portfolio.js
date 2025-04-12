import { loadIndex } from './portfolioIndex.js';
import { loadNewProject } from './portfolioNewProject.js';

export async function loadPortfolioPage(mode = "index") {
    switch (mode) {
        case "index":
            await loadIndex();
            break;
        case "edit":
            break;
        case "new":
            await loadNewProject();
            break;
    }
}



