import { loadBlogIndex } from "./blogIndex.js";
import { loadNewArticle } from "./blogNew.js";
import { loadEditArticle } from "./blogEdit.js";

export async function loadBlog(route = "index", id) {
    switch (route) {
        case "index": await loadBlogIndex();
            break;
        case "new": loadNewArticle();
            break;
        case "edit": await loadEditArticle(id);
            break;

    }
}
