use std::env;
use std::process;

use pinyin_tone_marks::do_convert;
use pinyin_tone_marks::get_file_text_as_string;
use pinyin_tone_marks::write_string_to_text_file;
use pinyin_tone_marks::Config;

fn main() {
    env_logger::init();

    log::info!("Starting Pinyin Converter...");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        log::error!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let lines = get_file_text_as_string(&config.infile);
    let lines = do_convert(&lines);
    write_string_to_text_file(&config.outfile, lines);

    log::info!("Conversion successful, wrote to file {}", &config.outfile);
}
