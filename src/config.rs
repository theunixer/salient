use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub double_dot_defence: bool,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:7878".to_string(),
            double_dot_defence: true,
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        confy::store("salient", None, self).unwrap()
    }
}
