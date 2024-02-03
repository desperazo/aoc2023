use std::fs;

pub fn read(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(data) => data
            .split('\n')
            .filter_map(|v| match v.len() {
                0 => None,
                _ => Some(v.to_string()),
            })
            .collect(),
        Err(_) => panic!("cannot read file from {}", path),
    }
}

pub fn read_string(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(data) => data,
        Err(_) => panic!("cannot read file from {}", path),
    }
}