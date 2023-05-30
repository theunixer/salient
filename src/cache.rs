use std::fs;

use crate::{
    https_tools::{Response, NOT_FOUND_STATUS},
    page::Page,
};

#[derive(Clone)]
pub struct Cache {
    pub pages: Vec<Page>,
    pub not_found: Response,
}

impl Cache {
    pub fn new() -> Self {
        let mut pages = Vec::new();
        let mut not_found = Response::new("Not found".to_string(), NOT_FOUND_STATUS);
        for file in fs::read_dir("./www").unwrap() {
            let file = file.unwrap().path();
            let path = file.as_path();

            if path.to_str().unwrap().contains("/not_found.html") {
                not_found = Response::new(fs::read_to_string(path).unwrap(), NOT_FOUND_STATUS)
            }

            let page = Page::new(path);
            pages.push(page);
        }
        Cache { pages, not_found }
    }
}