use std::env;
use rust_grep::{Config, search_one_file};


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

    if remaining_args.len() < 2 {
        eprintln!("Missing search filename");
        return;
    }


    config.pattern = pattern.to_string();
    config.filenames = remaining_args[1..]
                        .iter()
                        .map(|a| a.to_string())
                        .collect();

    let show_filename = config.filenames.len() > 1;

    for filename in &config.filenames 
    {
        search_one_file(filename, &config, show_filename)
    }
}


