mod brainfuck;

fn main() {
    let program = {
        let path_to_program = std::env::args().nth(1).expect("No program provided");

        let program_text =
            std::fs::read_to_string(path_to_program).expect("Unable to read program file");

        brainfuck::parse(&program_text)
    };

    let context = brainfuck::ExecutionContext::default();
    context.execute(&program);
}
