use std::io::Read;

fn main() {
    // hello world
    let program = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.---[->+++<]>.-[--->+<]>---.+++.------.--------.";

    let instructions = program.chars().collect::<Vec<_>>();

    let mut instruction_pointer = 0;

    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut data_pointer = 0;

    loop {
        if instruction_pointer >= instructions.len() {
            break;
        }

        let instruction = instructions[instruction_pointer];

        match instruction {
            '>' => data_pointer += 1,
            '<' => data_pointer -= 1,
            '+' => memory[data_pointer] = memory[data_pointer].wrapping_add(1),
            '-' => memory[data_pointer] = memory[data_pointer].wrapping_sub(1),
            '.' => print!("{}", char::from_u32(memory[data_pointer] as u32).unwrap()),
            ',' => std::io::stdin()
                .read_exact(&mut memory[data_pointer..=data_pointer])
                .unwrap(),
            '[' => {
                if memory[data_pointer] == 0 {
                    // skip forward to matching ']'
                    let mut balance = 1;

                    while balance > 0 {
                        instruction_pointer += 1;

                        match instructions[instruction_pointer] {
                            '[' => balance += 1,
                            ']' => balance -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if memory[data_pointer] != 0 {
                    // skip back to matching '['
                    let mut balance = 1;

                    while balance > 0 {
                        instruction_pointer -= 1;

                        match instructions[instruction_pointer] {
                            ']' => balance += 1,
                            '[' => balance -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        instruction_pointer += 1;
    }
}
