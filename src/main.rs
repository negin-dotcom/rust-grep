use std::{env, fs};
use rust_grep::{find_pattern, Config};


fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut remaining_args: Vec<&String> = Vec::new();
    let mut config = Config::create_default_config();


    for arg in args.iter().skip(1)
    {
        match arg.as_str()
        {
            "-n" => config.show_line_numbers = true,
            "-i" => config.ignore_case = true,
            "-v" => config.invert_match = true,
            "-c" => config.count_matching_lines = true,
            "-w" => config.whole_word = true,
            _ => {
                if arg.starts_with('-') 
                {
                    eprintln!("Unknown flag: {}", arg);
                    return; 

                } else {
                    remaining_args.push(arg);
                }
            }
        }
    }

    let pattern = match remaining_args.get(0) {
        Some(value) => value.as_str(),
        None => {
            eprintln!("Missing search pattern");
            return;
        }
    };

    let filename = match remaining_args.get(1) {
        Some(value) => value.as_str(), 
        None => {
            eprintln!("Missing search filename");
            return;
        }
    };

    config.pattern = pattern.to_string();
    config.filename = filename.to_string();


    let contents = match fs::read_to_string(&config.filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Could not read file: {}", error);
            return;
        }
    };

    let result = find_pattern(&contents, &config);
    if config.count_matching_lines {
            println!("{}", result.len());

    } else {
        for (idx, line) in result.iter()
            {
                if !config.show_line_numbers {
                    println!("{}", line);

                } else {
                    println!("{}: {}", idx, line);
                }
            }
    }
}
