use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() {
    generate_r_docs();
}

fn generate_r_docs() {
    let input_file_path = PathBuf::from("./harray.rs");
    let output_path = PathBuf::from("./");

    let mut hash: HashMap<String, Vec<String>> = HashMap::new();
    // Read the input file and filter to keep only lines starting with "///".
    let input_file = File::open(&input_file_path).unwrap();
    let mut key = String::new();
    let mut need_sep = false;
    for line in BufReader::new(input_file).lines().flatten() {
        if let Some(stripped) = line.trim_start().strip_prefix("///") {
            // skip first space.
            let filtered_line = if !stripped.is_empty() {
                stripped[1..].to_string()
            } else {
                stripped.to_string()
            };

            if !need_sep {
                key = filtered_line.clone();
                if key.contains(' ') {
                    panic!("the key must have only one token.");
                }
                hash.entry(key.clone()).or_insert_with(Vec::new);
            } else {
                hash.get_mut(&key).unwrap().push(filtered_line);
            }

            need_sep = true;
        } else if need_sep {
            //hash.get_mut(&key).unwrap().push("".into());
            need_sep = false;
        }
    }

    for (key, value) in hash {
        // header
        let key_lowercase = key.to_lowercase();
        let title = "title: ".to_string() + &key;
        let text = ["---", title.as_str(), "---"].join("\n");

        // Construct the final output text.
        let output_text = text.clone() + "\n\n" + &value.join("\n");

        // Construct the output file path as the input file path with a .md extension.
        let output_file_path = output_path.join(&key_lowercase).with_extension("qmd");

        // Write the output text to the output file.
        let mut output_file = File::create(output_file_path).unwrap();
        output_file.write_all(output_text.as_bytes()).unwrap();
    }
}
