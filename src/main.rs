use std::env;
use std::process;
mod data;

use pinyin_tone_marks::do_convert;
use pinyin_tone_marks::io::get_vec_of_strings_from_file;
use pinyin_tone_marks::io::write_vec_of_strings_to_file;
use pinyin_tone_marks::Config;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    log::info!("Starting Pinyin Converter...");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = get_vec_of_strings_from_file(&config.infile);
    let lines = do_convert(lines);
    write_vec_of_strings_to_file(&config.outfile, lines);

    log::info!("Conversion successful, wrote to file {}", &config.outfile);
}
