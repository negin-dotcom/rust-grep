use std::{env, fs};

fn main()
{
    let args: Vec<String> = env::args().collect();

    let mut i = 1;
    let mut show_line_numbers = false;
    let mut ignore_case = false; 

    while let Some(arg) = args.get(i) {
        match arg.as_str() {
            "-n" => show_line_numbers = true,
            "-i" => ignore_case = true,
            _ => break,
        }
        i += 1;
    }

    let pattern = match args.get(i) {
        Some(value) => value,
        None => {
            eprintln!("Missing search pattern");
            return;
        }
    };

    let filename = match args.get(i + 1) {
        Some(value) => value, 
        None => {
            eprintln!("Missing search filename");
            return;
        }
    };

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Could not read file: {}", error);
            return;
        }
    };

    let result = find_pattern(pattern, &contents, ignore_case);
    for (idx, line) in result.iter()
    {
        if !show_line_numbers {
            println!("{}", line);

        } else {
            println!("{}: {}", idx, line);
        }
    }

}


fn find_pattern<'a>(pattern: &str, contents: &'a str, ignore_case: bool) -> Vec<(usize, &'a str)> 
{
    let pattern_lower = pattern.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            if ignore_case {
                line.to_lowercase().contains(&pattern_lower)
            } else {
                line.contains(pattern)
            }
        })
        .map(|(idx, line)| (idx + 1, line))
        .collect()
}