use std::{env, fs::File, io::Read, process::exit};

mod lexer;
mod interpretter;

fn load_from_file(path: &str) -> Result<String, ()> {
    let mut fp = match File::open(path) {
        Ok(fp) => fp,
        Err(_) => return Err(())
    };
    let mut buf = String::new();
    match fp.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(_) => return Err(())
    }
}


fn main() -> Result<(), ()>{
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Insufficient arguments.");
        exit(1);
    }

    let content = load_from_file(&args[1])?;
    let mut program = interpretter::Program::new(&content);
    program.run();
    Ok(())
}
