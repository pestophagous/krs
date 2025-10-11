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

    // TODO: logger?
    println!("Hello {:#?}", args.inputfile);

    let extractor = krs_inputloader::extract::ExtractText::new(args.inputfile);

    let input_set = extractor.parse();
    let input_items = input_set.get_all_items();

    for item in input_items.iter() {
        println!("");
        println!("unique_id {}", item.unique_id);
        println!("answer {}", item.answer);
        println!("tags {:#?}", item.tags);
        println!("prompt {}", item.prompt);
    }
}
