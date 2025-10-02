use crate::krs_inputloader::ExtractText;

use clap::Parser;

mod krs_inputloader;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }
}

//fn main() {
//  println!("Hello World!");

// # FUTURE: pass in more than one input file
// p = os.path.normpath(os.path.join(os.getcwd(), sys.argv[1]))
// print(p)  # TODO: logger
// extractor = extract_text.ExtractText([p])
// inputset = extractor.parse()
//}
