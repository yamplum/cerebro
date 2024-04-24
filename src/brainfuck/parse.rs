use std::collections::HashMap;

#[derive(Debug)]
pub enum Instruction {
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

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        use Instruction::*;

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

pub struct Program {
    pub(super) instructions: Vec<Instruction>,
    pub(super) jump_targets: HashMap<usize, usize>,
}

pub fn parse(program_text: &str) -> Program {
    let instructions: Vec<_> = program_text.bytes().map(Instruction::from).collect();

    // A slight optimization: find pairs of jumps during the parsing pass and
    // record their positions in a lookup table. This has the added benefit of
    // validating that all jumps have a corresponding match prior to execution.
    let mut jump_stack = vec![];
    let mut jump_targets = HashMap::new();

    for (index, instruction) in instructions.iter().enumerate() {
        match instruction {
            Instruction::JumpForwardIfZero => {
                jump_stack.push(index);
            }

            Instruction::JumpBackUnlessZero => {
                let matching_jump_index = jump_stack.pop().expect(
                    "Malformed program: found backwards jump without a corresponding match",
                );

                jump_targets.insert(index, matching_jump_index);
                jump_targets.insert(matching_jump_index, index);
            }

            _ => {}
        }
    }

    if !jump_stack.is_empty() {
        panic!("Malformed program: found forward jump without a corresponding match");
    }

    Program {
        instructions,
        jump_targets,
    }
}
