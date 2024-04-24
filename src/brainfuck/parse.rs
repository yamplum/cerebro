#[derive(Debug)]
pub enum Instructions {
    MoveToNextCell,
    MoveToPreviousCell,
    IncrementCell,
    DecrementCell,
    WriteOutput,
    ReadInput,
    JumpForwardIfZero,
    JumpBackUnlessZero,
    NoOp,
}

impl From<u8> for Instructions {
    fn from(value: u8) -> Self {
        use Instructions::*;

        match value {
            b'>' => MoveToNextCell,
            b'<' => MoveToPreviousCell,
            b'+' => IncrementCell,
            b'-' => DecrementCell,
            b'.' => WriteOutput,
            b',' => ReadInput,
            b'[' => JumpForwardIfZero,
            b']' => JumpBackUnlessZero,
            _ => NoOp,
        }
    }
}

pub fn parse(program: &str) -> Vec<Instructions> {
    program.bytes().map(From::from).collect()
}
