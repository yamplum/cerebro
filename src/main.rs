mod brainfuck;

fn main() {
    // hello world
    const PROGRAM: &str = "-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>-----.---[->+++<]>.-[--->+<]>---.+++.------.--------.";

    let program = brainfuck::parse(PROGRAM);

    let context = brainfuck::ExecutionContext::default();
    context.execute(&program);
}
