//use crate::krs_base::set::Set; // oh!
use crate::krs_inputloader::ExtractText; // #[warn(unused_imports)] can i do as error?

use clap::Parser; // https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html

mod krs_base;
mod krs_inputloader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Either exactly one "metafile", or 1+ problemset files
    inputfile: Vec<String>,
}

fn main() {
    // FUTURE: we want config file and/or CLI args (as the krs-b has), so check out:
    //    https://stackoverflow.com/questions/55133351/is-there-a-way-to-get-clap-to-use-default-values-from-a-file
    //    https://www.reddit.com/r/rust/comments/1ecnriw/command_line_env_vars_and_configs/
    //    https://github.com/clap-rs/clap/discussions/2763#discussioncomment-7633613
    let args = Args::parse();

    let input_set = krs_base::set::Set::new();
    let mut input_items = input_set.get_all_items();

    // extractor = extract_text.ExtractText([p])
    // inputset = extractor.parse()

    // print(p)  # TODO: logger

    println!("Hello {:#?}", args.inputfile);

    for item in input_items.iter() {
        println!("");
        println!("unique_id {}", item.unique_id);
        println!("answer {}", item.answer);
        println!("tags {:#?}", item.tags);
        println!("prompt {}", item.prompt);
    }
}
