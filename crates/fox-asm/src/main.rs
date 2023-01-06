mod tokenizer;
mod parser;
mod asm;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Expected 1 argument");
        return;
    }

    let input_filename = std::path::Path::new(&args[1]);
    let output_filename = input_filename.with_extension("bin");
    println!("Writing to {}", output_filename.display());

    let input = std::fs::read_to_string(input_filename).unwrap();

    let tokens = tokenizer::tokenize(&input);
    let ast = parser::parse(&tokens);

    let mut asm = asm::Assembler::new();
    asm.assemble(&ast);

    //println!("Tokens: {:x?}", tokens);
    //println!("{:x?}", ast);
    //println!("Asm: {:x?}", asm.data());

    std::fs::write(output_filename, asm.data()).unwrap();
}
