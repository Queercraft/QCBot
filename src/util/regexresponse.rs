use std::sync::{Arc, RwLock};
use regex::Regex;

use crate::Config;


pub fn regexresponse(config: Arc<RwLock<Config>>, content: String) -> Option<(String, String)> {
    for (regex, response) in &config.read().unwrap().regex_responses {
        if Regex::new(&regex.to_lowercase()).unwrap().is_match(&content.to_lowercase()) {
            return Some((response.to_string(), regex.to_string()));
        }
    }
    None
}