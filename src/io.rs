use std::fs;

pub fn get_vec_of_strings_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect(&format!("Unable to read from file {}", filename))
}

pub fn write_vec_of_strings_to_file(filename: &str, output: String) {
    fs::write(filename, output).expect(&format!("Unable to write to file {}", filename));
}
