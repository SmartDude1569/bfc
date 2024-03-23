use std::env;
use std::process;
use std::fs;

mod tape;
use tape::Tape;

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
    let mut src = fs::read_to_string(src_name)
        .expect("couldn't read the file");
    println!("executing {}", src_name);
    let mut tape = Tape::<i8>::new(30000);
    for instruction in unsafe { src.as_mut_vec() } {
        match instruction {
            b'>' => {
                tape.move_right();
                ()
            }
            b'<' => {
                tape.move_left();
                ()
            }
            b'+' => {
                tape.increment();
                ()
            }
            b'-' => {
                tape.decrement();
                ()
            }
            b'.' => {
                print!("{}", tape.get());
                ()
            }
            b',' => {
                // anyone who uses input in their program is a pussy
                ()
            }
            b'[' => {
                // fuck you im not gonna implement this shit
                ()
            }
            b']' => {
                // ill implement these sanity-fuckers later
                ()
            }
            _ => {
                ()
            }
        }
    }
}