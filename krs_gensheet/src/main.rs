//use crate::krs_base::set::Set; // oh!
use crate::krs_inputloader::ExtractText; // #[warn(unused_imports)] can i do as error?

use clap::Parser;

mod krs_base;
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
    // FUTURE: we want config file and/or CLI args (as the krs-b has), so check out:
    //    https://stackoverflow.com/questions/55133351/is-there-a-way-to-get-clap-to-use-default-values-from-a-file
    //    https://www.reddit.com/r/rust/comments/1ecnriw/command_line_env_vars_and_configs/
    //    https://github.com/clap-rs/clap/discussions/2763#discussioncomment-7633613
    let args = Args::parse();

    let input_set = krs_base::set::Set::new();
    let mut input_items = input_set.get_all_items();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    for item in input_items.iter() {
        println!("");
        println!("unique_id {}", item.unique_id);
        println!("answer {}", item.answer);
        println!("tags {:#?}", item.tags);
        println!("prompt {}", item.prompt);
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
