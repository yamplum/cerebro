use std::io::Read;

use super::parse::Instructions;

pub struct ExecutionContext {
    program_counter: usize,

    cell_index: usize,
    cells: [u8; 30_000],
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            program_counter: 0,
            cell_index: 0,
            cells: [0; 30_000],
        }
    }
}

impl ExecutionContext {
    pub fn execute(mut self, program: &[Instructions]) {
        loop {
            if self.program_counter >= program.len() {
                break;
            }

            let instruction = &program[self.program_counter];

            use Instructions::*;
            match instruction {
                MoveToNextCell => self.cell_index += 1,

                MoveToPreviousCell => self.cell_index -= 1,

                IncrementCell => *self.cell() = self.cell().wrapping_add(1),

                DecrementCell => *self.cell() = self.cell().wrapping_sub(1),

                WriteOutput => print!("{}", char::from(*self.cell())),

                ReadInput => {
                    let destination = &mut self.cells[self.cell_index..=self.cell_index];

                    std::io::stdin().read_exact(destination).unwrap()
                }

                JumpForwardIfZero => {
                    if *self.cell() == 0 {
                        // skip forward to matching ']'
                        let mut balance = 1;

                        while balance > 0 {
                            self.program_counter += 1;

                            match program[self.program_counter] {
                                JumpForwardIfZero => balance += 1,
                                JumpBackUnlessZero => balance -= 1,
                                _ => {}
                            }
                        }
                    }
                }

                JumpBackUnlessZero => {
                    if *self.cell() != 0 {
                        // skip back to matching '['
                        let mut balance = 1;

                        while balance > 0 {
                            self.program_counter -= 1;

                            match program[self.program_counter] {
                                JumpBackUnlessZero => balance += 1,
                                JumpForwardIfZero => balance -= 1,
                                _ => {}
                            }
                        }
                    }
                }

                NoOp => {}
            }

            self.program_counter += 1;
        }
    }

    /// A mutable reference to the cell currently pointed to by `cell_index`.
    ///
    /// Assumes `cell_index` cannot be out of bounds.
    fn cell(&mut self) -> &mut u8 {
        &mut self.cells[self.cell_index]
    }
}
