use std::{fs, path::Path};

use crate::https_tools::{Response, OK_STATUS};

#[derive(Clone)]
pub struct Page {
    pub path: String,
    pub response: Response,
}

impl Page {
    pub fn new(location: &Path) -> Self {
        let path = location.to_str().unwrap().split("www").nth(1).unwrap();
        let path = "./www".to_owned() + path;
        let response: Response = Response::new(fs::read_to_string(location).unwrap(), OK_STATUS);
        Page { path, response }
    }
}
