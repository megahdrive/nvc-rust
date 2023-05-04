use std::{path::Path, env};

pub fn path_exists(path: &str) -> bool {
    return Path::new(path).is_dir();
}

pub fn get_cwd() -> String {
    return String::from(env::current_dir().unwrap().to_str().unwrap());
}