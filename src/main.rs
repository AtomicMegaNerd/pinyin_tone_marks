#![warn(clippy::all, clippy::pedantic)]

use std::env;
use std::fs;

use text_colorizer::Colorize;

use pinyin_tone_marks::do_convert;

///
/// # Config
/// This object holds the arguments passed into the program.
/// The two arguments are:
/// - infile - The path to the input text file to convert.
/// - outfile - The destination file where the coverted text is written.
struct Config {
    infile: String,
    outfile: String,
}

impl Config {
    fn new(args: std::env::Args) -> Config {
        let args: Vec<String> = args.skip(1).collect();
        if args.len() != 2 {
            panic!(
                "{} wrong number of arguments: expected 2, got {}",
                "Error:".red().bold(),
                args.len(),
            );
        }

        Config {
            infile: args[0].clone(),
            outfile: args[1].clone(),
        }
    }
}

fn get_file_text_as_string(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| {
        panic!(
            "{} Unable to read from file {}",
            "Error:".red().bold(),
            filename
        )
    })
}

fn write_string_to_text_file(filename: &str, output: String) {
    fs::write(filename, output).unwrap_or_else(|_| {
        panic!(
            "{} Unable to write to file {}",
            "Error:".red().bold(),
            filename
        )
    });
}

fn main() {
    println!("{}", "Starting Pinyin Converter...".green());

    let config = Config::new(env::args());
    let lines = get_file_text_as_string(&config.infile);
    let lines = do_convert(&lines);
    write_string_to_text_file(&config.outfile, lines);

    println!(
        "Conversion successful, wrote to file {}",
        &config.outfile.green()
    );
}
