use std::ops::Index;
use std::str::Chars;
use project::{Position, ProjectFile};
use tokens::Token;

const VARIABLES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZαβγδεϝζηθϑκϰλμξϖρϱσςϕφψωΓΔϜΘΛΞΟΠΡΣΦΨΩπτ";
const VAR_SUBS: &str = "ΑAΒBΕEΖZΗHΙIΚKΜMΝNΤTΥYΧX";
// reserved     ϵ
// identicals   ηινουχ ΑΒΕΖΗΙΚΜΝΤΥΧ

const NUMERALS: &str = "0123456789";

pub struct Project<'a> {
    source: &'a Chars<'a>,
    file: &'a ProjectFile<'a>,
    tokens: Vec<Token<'a>>,

    last_read: Option<char>,
    current_index: usize
}

fn variable(project: &mut Project) {
    project.tokens.push(Token::Variable {
        name: project.last_read.unwrap(), position: Position::before(project.file, project.current_index)
    });
}

fn read(project: &mut Project) -> Option<char> {
    let last_read = project.source.next();
    project.current_index += 1;
    last_read
}

fn lex(project_file: &ProjectFile) {
    let mut project = Project {
        source: &project_file.read().chars(),
        file: project_file,
        tokens: vec![],
        last_read: None,
        current_index: 0
    };

    loop {
        if read(&mut project).is_none() {
            break
        }

        if VARIABLES.contains(project.last_read.unwrap()) {
            variable(&mut project);
        }
    }
}