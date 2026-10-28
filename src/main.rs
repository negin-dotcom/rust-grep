use std::{env, fs};

fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut remaining_args: Vec<&String> = Vec::new();

    let mut invert_match = false;

    let mut show_line_numbers = false;
    let mut ignore_case = false; 
    let mut count_matching_lines = false;


    for arg in args.iter().skip(1)
    {
        match arg.as_str()
        {
            "-n" => show_line_numbers = true,
            "-i" => ignore_case = true,
            "-v" => invert_match = true,
            "-c" => count_matching_lines = true,
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

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Could not read file: {}", error);
            return;
        }
    };

    let result = find_pattern(pattern, &contents, ignore_case,
                                                  invert_match);
    if count_matching_lines {
            println!("{}", result.len());

    } else {
        for (idx, line) in result.iter()
            {
            
            if !show_line_numbers {
                println!("{}", line);

            } else {
                println!("{}: {}", idx, line);
            }
        }
    }

    

}


fn find_pattern<'a>(pattern: &str, contents: &'a str, ignore_case: bool,
                    invert_match: bool) -> Vec<(usize, &'a str)> 
{
    let pattern_lower = pattern.to_lowercase();

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let matches = if ignore_case {
                line.to_lowercase().contains(&pattern_lower)
            } else {
                line.contains(pattern)
            };

            matches != invert_match
        })
        .map(|(idx, line)| (idx + 1, line))
        .collect()
}



#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn basic_search()
    {
        let contents = "hello rust\nhello python\nhello rust again";

        let result = find_pattern("rust", contents, false);

        assert_eq!(
            result,
            vec![(1, "hello rust"), (3, "hello rust again")]
        );
    }
}