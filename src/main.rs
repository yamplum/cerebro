use std::io::Read;

mod brainfuck {

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
}

fn main() {
    // hello world
    const PROGRAM: &str = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.---[->+++<]>.-[--->+<]>---.+++.------.--------.";

    let instructions: Vec<brainfuck::Instructions> = PROGRAM.bytes().map(From::from).collect();

    let mut instruction_pointer = 0;

    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut data_pointer = 0;

    loop {
        if instruction_pointer >= instructions.len() {
            break;
        }

        let instruction = &instructions[instruction_pointer];

        use brainfuck::Instructions::*;
        match instruction {
            MoveToNextCell => data_pointer += 1,
            MoveToPreviousCell => data_pointer -= 1,
            IncrementCell => memory[data_pointer] = memory[data_pointer].wrapping_add(1),
            DecrementCell => memory[data_pointer] = memory[data_pointer].wrapping_sub(1),
            WriteOutput => print!("{}", char::from(memory[data_pointer])),
            ReadInput => std::io::stdin()
                .read_exact(&mut memory[data_pointer..=data_pointer])
                .unwrap(),
            JumpForwardIfZero => {
                if memory[data_pointer] == 0 {
                    // skip forward to matching ']'
                    let mut balance = 1;

                    while balance > 0 {
                        instruction_pointer += 1;

                        match instructions[instruction_pointer] {
                            JumpForwardIfZero => balance += 1,
                            JumpBackUnlessZero => balance -= 1,
                            _ => {}
                        }
                    }
                }
            }
            JumpBackUnlessZero => {
                if memory[data_pointer] != 0 {
                    // skip back to matching '['
                    let mut balance = 1;

                    while balance > 0 {
                        instruction_pointer -= 1;

                        match instructions[instruction_pointer] {
                            JumpBackUnlessZero => balance += 1,
                            JumpForwardIfZero => balance -= 1,
                            _ => {}
                        }
                    }
                }
            }
            NoOp => {}
        }

        instruction_pointer += 1;
    }
}
