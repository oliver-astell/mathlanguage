use std::fs;
use lexer::Project;

pub struct ProjectFile<'a> {
    pub(crate) path: &'a str,
}

impl ProjectFile {
    pub fn read(&self) -> String {
        fs::read_to_string(self.path).expect("Implement false file case")
    }
}

pub struct Position<'a> {
    pub(crate) file: &'a ProjectFile<'a>,
    pub(crate) index: usize,
    pub(crate) end_index: usize,
}

impl <'a> Position<'a> {
    pub fn from(file: &'a ProjectFile, starting: usize, ending: usize) -> Self {
        Position {
            file,
            index: starting,
            end_index: ending
        }
    }
    pub fn at(file: &'a ProjectFile, at: usize) -> Self {
        Position {
            file,
            index: at,
            end_index: at+1
        }
    }
    pub fn before(file: &'a ProjectFile, at: usize) -> Self {
        Position {
            file,
            index: at-1,
            end_index: at
        }
    }
}