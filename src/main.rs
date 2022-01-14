use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn is_markdown_file(path: &Path, input_ext: &str) -> bool {
    if let Some(ext) = path.extension() {
        return ext == input_ext;
    } else {
        return false;
    }
}

fn do_replace(content: &str) -> String {
    let mut result = String::new();

    let mut past_char = '\0';
    let mut in_codeblocks = false;
    let mut in_math = false;
    for c in content.chars() {
        // println!("{}", c);
        if '`' == c {
            if '`' == past_char {
                // do nothing: skip muti-backtick
            } else {
                in_codeblocks = !in_codeblocks;
            }
        } else if in_codeblocks {
            // do nothing: skip codeblocks
        } else if '$' == c {
            if '$' != past_char {
                in_math = !in_math;
            }
        } else if in_math {
            if '_' == c {
                result.push('\\');
            } else if '*' == c {
                result.push('\\');
            } else if '\\' == c {
                result.push('\\');
            }
        }
        result.push(c);
        past_char = c;
    }
    return result;
}

fn dfs(path: &PathBuf, input_ext: &str, output_ext: &str) {
    if path.is_dir() {
        let paths = fs::read_dir(path).expect("Directory not exists");
        for path in paths {
            let path = path.unwrap().path();
            dfs(&path, input_ext, output_ext);
        }
    } else if is_markdown_file(&path, input_ext) {
        println!("{}", path.display());
        let contents = fs::read_to_string(path).expect("Unable to read file");
        let after = do_replace(&contents);
        let output_path = path.with_extension(output_ext);
        fs::write(output_path, after).expect("Unable to write the file");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!(r#"usage: preprocesser-md-tex <path>
   or  preprocesser-md-tex <path> <intput_ext> <output_ext>

   default extension is "md"."#);
    } else {
        let [input_ext, output_ext] = if args.len() < 4 {
            ["md", "md"]
        } else {
            [args[2].as_str(), args[3].as_str()]
        };

        let target_path = PathBuf::from(&args[1]);

        if target_path.is_dir() || is_markdown_file(&target_path, input_ext) {
            dfs(&target_path, input_ext, output_ext);
        } else {            
            println!("Not a directory or md file.");
        }
    }
}
