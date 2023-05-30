use std::fs;

use crate::{
    cache::Cache,
    response::{Response, NOT_FOUND_STATUS, OK_STATUS},
};

pub fn format_path(mut path: &str, double_dot_defence: &bool) -> String {
    if *double_dot_defence && path.contains("..") {
        path = "not_found";
    }
    
    if path == "/" {
        path = "/index.html";
    }

    if path.contains('.') {
        format!("./www{path}")
    } else {
        format!("./www{path}.html")
    }
}

pub fn page_from_file(path: &String) -> Response {
    match fs::read_to_string(path) {
        Ok(result) => Response::new(result, OK_STATUS),
        Err(_) => Response::new(
            fs::read_to_string("./www/not_found.html").unwrap_or("Not found.".to_string()),
            NOT_FOUND_STATUS,
        ),
    }
}

pub fn page_from_cache(cache: &Cache, path: &String) -> Response {
    let mut demanded_page = None;
    for page in &cache.pages {
        if page.path == *path {
            demanded_page = Some(page.response.clone());
        }
    }
    demanded_page.unwrap_or(cache.not_found.clone())
}
