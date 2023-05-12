use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() {
    generate_r_docs();
}

fn generate_r_docs() {
    let files = ["./harray.rs", "./hdatatype.rs"];
    let mut hash: HashMap<String, Vec<String>> = HashMap::new();
    let output_path = PathBuf::from("../../../docs/docswebsite/docs");
    let github_folder =
        "https://www.github.com/daniellga/harmonium/blob/master/r-harmonium/src/rust/src/";

    for file in files {
        let input_file_path = PathBuf::from(file);

        // Read the input file and filter to keep only lines starting with "///".
        let input_file = File::open(&input_file_path).unwrap();
        let mut key = String::new();
        let mut last_line_was_comment = false;
        let mut skip_comment_chunk = false;
        let mut counter: i32 = -1;
        for (line_counter, line) in BufReader::new(input_file).lines().flatten().enumerate() {
            let line_trimmed = line.trim_start();

            if let Some(stripped) = line_trimmed.strip_prefix("///") {
                counter += 1;
                if skip_comment_chunk {
                    continue;
                }

                // skip first space.
                let filtered_line = stripped.strip_prefix(' ').unwrap_or(stripped).to_string();

                // associate with key in first line of comment chunk.
                if !last_line_was_comment {
                    key = filtered_line.clone();
                    if key.contains(' ') {
                        skip_comment_chunk = true;
                        continue;
                    }
                    hash.entry(key.clone()).or_insert_with(Vec::new);
                    last_line_was_comment = true;
                } else {
                    hash.get_mut(&key).unwrap().push(filtered_line);
                }
            } else {
                // add the source text. Code on github must be updated.
                if last_line_was_comment {
                    if line_trimmed.contains("fn") && line_trimmed.contains('{') {
                        let vec = hash.get_mut(&key).unwrap();
                        let len = vec.len();
                        let elem = &mut vec[len - counter as usize + 2];
                        elem.pop();
                        let path = files
                            .iter()
                            .find(|&&x| x.contains(&key.to_lowercase()))
                            .unwrap()
                            .strip_prefix("./")
                            .unwrap();

                        let source = "<span style=\"float: right;\"> [source](".to_string()
                            + github_folder
                            + path
                            + "#L"
                            + &(line_counter + 1).to_string()
                            + ") </span> \\";
                        elem.push_str(&source);
                    }
                    counter = -1;
                }

                skip_comment_chunk = false;
                last_line_was_comment = false;
            }
        }
    }

    output_file(hash, output_path)
}

fn output_file(hash: HashMap<String, Vec<String>>, output_path: PathBuf) {
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
