fn is_markdown_file(file_path: &std::path::Path) -> bool {
    if let Some(ext) = file_path.extension() {
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

fn work(file_path: &std::path::Path) {
    let contents = std::fs::read_to_string(file_path).expect("Unable to read file");
    let after = do_replace(&contents);
    std::fs::write(file_path, after).expect("Unable to write the file");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        let arg_path = std::path::Path::new(&args[1]);
        if arg_path.is_file() {
            work(arg_path);
        }
    }

    let walker = if args.len() >= 2 {
        walkdir::WalkDir::new(&args[1])
    } else {
        walkdir::WalkDir::new(std::env::current_dir().unwrap())
    };

    for entry in walker {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if !is_markdown_file(file_path) {
            continue;
        }

        println!("{}", file_path.display());
        work(file_path);
    }
}
