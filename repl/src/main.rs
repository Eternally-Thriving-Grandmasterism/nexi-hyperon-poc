use hyperon::{Interpreter, Space};
use nexi_integration::nexi_mercy_space;

fn main() {
    let mercy_space = nexi_mercy_space();
    let mut interp = Interpreter::new(mercy_space);

    // Simple REPL loop (expand with full Hyperon REPL later)
    println!("NEXi-Hyperon Mercy REPL - Type MeTTa expressions");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" { break; }
        
        let results = interp.eval(&input);
        println!("{:?}", results);
    }
}
