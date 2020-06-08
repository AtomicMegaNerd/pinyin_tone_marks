use std::fs;

pub fn get_vec_of_strings_from_file(filename: &str) -> Vec<String> {
    match fs::read_to_string(filename) {
        Ok(data) => {
            let mut s_vec: Vec<String> = Vec::new();
            let lines = data.lines();
            for line in lines {
                s_vec.push(String::from(line));
            }
            s_vec
        }
        Err(e) => panic!("Unable to open file due to error {:?}", e),
    }
}

pub fn write_vec_of_strings_to_file(filename: &str, text_vec: Vec<String>) {
    let outputstr = text_vec.join("\n");
    fs::write(filename, outputstr).expect("Unable to write file");
}
