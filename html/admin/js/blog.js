import { loadBlogIndex } from "./blogIndex.js";
import { loadNewArticle } from "./blogNew.js";

export async function loadBlog(route = "index") {
    switch (route) {
        case "index": await loadBlogIndex();
            break;
        case "new": loadNewArticle();
            break;

    }
}
