use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn is_markdown_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        return ext == "md";
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

fn dfs(path: &PathBuf) {
    if path.is_dir() {
        let paths = fs::read_dir(path).expect("Directory not exists");
        for path in paths {
            let path = path.unwrap().path();
            dfs(&path);
        }
    } else if is_markdown_file(&path) {
        println!("{}", path.display());
        let contents = fs::read_to_string(path).expect("Unable to read file");
        let after = do_replace(&contents);
        fs::write(path, after).expect("Unable to write the file");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: preprocesser-md-tex <path>");
    } else {
        let target_path = PathBuf::from(&args[1]);
        if target_path.is_dir() || is_markdown_file(&target_path) {
            dfs(&target_path);
        } else {
            println!("Not a directory or md file.");
        }
    }
}
