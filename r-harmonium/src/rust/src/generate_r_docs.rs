use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn main() {
    let files = vec![
        "./harray.rs",
        "./haudio.rs",
        "./hmatrix.rs",
        "./hdatatype.rs",
        "./haudiosink.rs",
        "./hresamplertype.rs",
        "./hwindow.rs",
        "./hmetadatatype.rs",
        "./hpolynomialdegree.rs",
        "../../../R/hconfig.R",
    ];

    let gh = "https://www.github.com/daniellga/harmonium/blob/master/r-harmonium/src/rust/src/";

    let mut hash: HashMap<String, Vec<String>> = HashMap::new();

    generate_r_docs(files, gh, &mut hash);

    let output_path = PathBuf::from("../../../docs/docs");

    output_file(hash, output_path)
}

// Currently it may give a bug if 2 methods impl for the same struct are on different files,
// depending on the order of the files on the list below. Try to reorder the vec in a way that
// if the code chunk contains "# Methods" it will be swapped to the vec's first position.
fn generate_r_docs(files: Vec<&str>, github_folder: &str, hash: &mut HashMap<String, Vec<String>>) {
    for file in &files {
        // Read the input file and filter to keep only lines starting with "///".
        let input_file = File::open(file).unwrap();
        let mut key = String::new();
        let mut last_line_was_comment = false;
        let mut skip_comment_chunk = false;

        // counts the line in a code chunk
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

                // associate with key in first line of comment chunk. keys are identifiable by a 1 word line.
                if !last_line_was_comment {
                    key = filtered_line.clone();
                    // key should have only one word
                    if key.contains(' ') {
                        skip_comment_chunk = true;
                        continue;
                    }
                    hash.entry(key.clone()).or_insert_with(Vec::new);
                    last_line_was_comment = true;
                } else {
                    hash.get_mut(&key).unwrap().push(filtered_line);
                }
            } else if let Some(stripped) = line_trimmed.strip_prefix("###") {
                counter += 1;
                if skip_comment_chunk {
                    continue;
                }

                // skip first space.
                let filtered_line = stripped.strip_prefix(' ').unwrap_or(stripped).to_string();

                // associate with key in first line of comment chunk. keys are identifiable by a 1 word line.
                if !last_line_was_comment {
                    key = filtered_line.clone();
                    // key should have only one word
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
                if last_line_was_comment
                    && line_trimmed.contains("fn")
                    && line_trimmed.contains('{')
                {
                    let vec = hash.get_mut(&key).unwrap();
                    let len = vec.len();
                    let elem = &mut vec[len - counter as usize + 2];
                    elem.pop();

                    let filename = files
                        .iter()
                        .find(|&&x| x.contains(&key.to_lowercase()))
                        .unwrap();

                    let source = "<span style=\"float: right;\"> [source](".to_string()
                        + github_folder
                        + filename
                        + "#L"
                        + &(line_counter + 1).to_string()
                        + ") </span> \\";
                    elem.push_str(&source);
                }

                counter = -1;
                skip_comment_chunk = false;
                last_line_was_comment = false;
            }
        }
    }
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
