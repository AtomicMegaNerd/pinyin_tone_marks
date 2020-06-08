mod data;
pub mod io;

pub use io::get_vec_of_strings_from_file;
pub use io::write_vec_of_strings_to_file;

use data::PinyinData;
use data::TONES;
use data::VOWELS;

pub struct Config {
    pub infile: String,
    pub outfile: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let infile = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing argument input file"),
        };

        let outfile = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing argument output file"),
        };

        Ok(Config { infile, outfile })
    }
}

/// This method is the meat of the program.
///
/// This is the algorithm we are going to use:
///
///    For each character in each line:
///    1. Start a search string as a buffer.
///    2. If a character is an initial consonant ignore it.
///    3. If a character is a vowel or an ending consonant append it to the search
///   string.
///    4. If a character is a tone indicator (1,2,3, or 4) that's the terminator for a
///   search.  in that case, add it as the final character to the search string. Then
///    search the 4, 3, 2, or 1 letter dictionaries for a match.  If a match is found
///    replace it.
///
///    It is very important that we search the bigger dicts before the smaller ones
///    because the smaller ones will also match on longer sequences and we do not want
///    that.
///
///    Append modified lines to a new list called converted which the caller can use
///    to output pinyin tone marks.
pub fn do_convert(text: Vec<String>) -> Vec<String> {
    let data = PinyinData::new();
    let mut converted: Vec<String> = Vec::new();

    for mut line in text {
        // This string contains valid Pinyin initials and finals which
        // contain at least a vowel and a number...
        let mut search_str: String = String::new();

        // We have to clone line here because we mutate it below...
        for ch in line.clone().chars() {
            // Add all letters that are not initial consonants to the search
            // string. Spaces and punctuation will be ignored.
            if ch.is_alphabetic() {
                if !VOWELS.contains(ch) && search_str.len() == 0 {
                    continue;
                }
                search_str.push(ch);
            }
            // All of our search strings will end with the tone indicator which
            // will be 1, 2, 3, or 4. Note that this number may come after a vowel
            // or a consonant but we cover both cases in our four match_map HashMaps.
            else if TONES.contains(ch) {
                // Don't forget to append the number as they are in the keys
                // to our HashMaps...
                search_str.push(ch);
                log::debug!("Buffer = {}", &search_str);

                // Make sure that we always search the bigger dicts first otherwise
                // we will not catch the longer sequences. The search string is
                // compared with the keys from each dict. If it matches a key in
                // any of our dicts the value from that dict will be used to
                // replace the text in the line.
                let key = search_str.as_str();
                if data.match_map4.contains_key(key) {
                    line = line.replace(&search_str, &data.match_map4[key]);
                } else if data.match_map3.contains_key(key) {
                    line = line.replace(&search_str, &data.match_map3[key]);
                } else if data.match_map2.contains_key(key) {
                    line = line.replace(&search_str, &data.match_map2[key]);
                } else if data.match_map1.contains_key(key) {
                    line = line.replace(&search_str, &data.match_map1[key]);
                }
                search_str.clear();
            } else {
                // Reset the search string if we are onto a new word...
                search_str.clear();
            }
        }

        log::debug!("Converted line: {}", &line);
        converted.push(line);
    }
    converted
}
