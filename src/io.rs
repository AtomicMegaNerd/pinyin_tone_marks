use std::fs;

pub fn get_vec_of_strings_from_file(filename: &str) -> Vec<String> {
    let mut s_vec: Vec<String> = Vec::new();
    let lines =
        fs::read_to_string(filename).expect(&format!("Unable to read from file {}", filename));
    for line in lines.lines() {
        s_vec.push(String::from(line));
    }
    s_vec
}

pub fn write_vec_of_strings_to_file(filename: &str, text_vec: Vec<String>) {
    fs::write(filename, text_vec.join("\n"))
        .expect(&format!("Unable to write to file {}", filename));
}
