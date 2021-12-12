use std::fs;

pub fn get_file_text_as_string(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|_| panic!("Unable to read from file {}", filename))
}

pub fn write_string_to_text_file(filename: &str, output: String) {
    fs::write(filename, output).unwrap_or_else(|_| panic!("Unable to write to file {}", filename));
}
