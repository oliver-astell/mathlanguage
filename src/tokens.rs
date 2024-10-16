use project::Position;

pub enum Token<'a> {
    Variable { position: Position<'a>, name: char },
    Number { position: Position<'a>, name: char },
}