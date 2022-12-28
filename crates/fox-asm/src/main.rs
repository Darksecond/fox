mod tokenizer;
mod parser;
mod asm;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("Expected 2 arguments");
        return;
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();

    let tokens = tokenizer::tokenize(&input);
    let ast = parser::parse(&tokens);

    let mut asm = asm::Assembler::new();
    asm.assemble(&ast);

    //println!("Tokens: {:x?}", tokens);
    //println!("{:x?}", ast);
    //println!("Asm: {:x?}", asm.data());

    std::fs::write(&args[2], asm.data()).unwrap();
}
