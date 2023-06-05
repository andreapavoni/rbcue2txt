use std::env;

mod cue_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide the path to the .cue file as a command-line argument");
        return;
    }

    if let Ok(outputs) = cue_parser::run(&args[1]) {
        for t in outputs.into_iter() {
            println!("- {}: {} - {}\n", t.time, t.artist, t.title);
        }
    }
}
