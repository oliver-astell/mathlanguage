use std::fs;

pub struct Project<'a> {
    path: &'a str,
}

impl Project {
    pub fn read(&self) -> String {
        fs::read_to_string(self.path).expect("Implement false file case")
    }
}