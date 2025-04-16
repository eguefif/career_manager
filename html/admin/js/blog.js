import { loadBlogIndex } from "./blogIndex.js";
import { loadNewArticle } from "./blogNew.js";

export function loadBlog(route = "index") {
    switch (route) {
        case "index": loadBlogIndex();
            break;
        case "new": loadNewArticle();
            break;

    }
}
