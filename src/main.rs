use std::env;

mod rb_cue;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        println!("Please provide the path to the .cue file.");
        println!("Optionally, you can provide the desired format as a second argument");
        return;
    }

    let format = if args.len() == 3 { &args[2] } else { "- [%T] %A - %N" };

    if let Ok(cue_sheet) = rb_cue::CueSheet::new(&args[1]) {
        for t in cue_sheet.playlist.into_iter() {
            println!("{}", t.format(&format));
        }
    }
}
