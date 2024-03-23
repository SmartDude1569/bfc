use std::env;
use std::process;

fn print_help(args: &Vec<String>) {
    let bin_name = &args[0];
    println!("{} [brainfuck source file]", bin_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("invalid usage");
        print_help(&args);
        process::exit(1);
    }
    let src_name = &args[1];
    println!("executing {}", src_name);
}