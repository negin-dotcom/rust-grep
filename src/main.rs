use std::{env, fs};


struct Config
{
    pattern: String,
    filename: String,
    ignore_case: bool, 
    invert_match: bool,
    whole_word: bool,
    show_line_numbers: bool,
    count_matching_lines: bool,
}

impl Config 
{
    fn create_default_config() -> Config
    {
        Config {
            pattern: String::new(),
            filename: String::new(),
            ignore_case: false,
            invert_match: false,
            whole_word: false,
            show_line_numbers: false,
            count_matching_lines: false,
        }
    }
}


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


fn is_word_char(c: char) -> bool 
{
    c.is_alphanumeric() || c == '_'
}

fn contains_whole_word(line: &str, pattern: &str) -> bool 
{
    if pattern.is_empty()
    {
        return true;
    }
     
    for (start, _) in line.match_indices(pattern) 
    {
        let end = start + pattern.len();

        let before_ok = line[..start]
                            .chars()
                            .next_back()
                            .map_or(true, |c| !is_word_char(c));
        
        let after_ok = line[end..]
                            .chars()
                            .next()
                            .map_or(true, |c| !is_word_char(c));
        
        if before_ok && after_ok 
        {
            return true;
        }
    }    
    false       
}


fn find_pattern<'a>(contents: &'a str, config: &Config) -> Vec<(usize, &'a str)> 
{
    let pattern_lower = if config.ignore_case { 
        config.pattern.to_lowercase() 

    } else {
         String::new() 
    }; 

    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let matches = match (config.ignore_case, config.whole_word)
            {
                (true, true) => {
                    let line_lower = line.to_lowercase();
                    contains_whole_word(&line_lower, &pattern_lower)
                },
                (true, false) => {
                    let line_lower = line.to_lowercase();
                    line_lower.contains(&pattern_lower)
                },
                (false, true) => contains_whole_word(line, &config.pattern),
                (false, false) => line.contains(&config.pattern),
            };

            matches != config.invert_match
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
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(1, "hello rust"), (3, "hello rust again")]
        );
    }

    #[test]
    fn case_insensitive_search()
    {
        let contents = "hello rust\nhello python\nhello RUST again";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.ignore_case = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(1, "hello rust"), (3, "hello RUST again")]
        );                       
    }

    #[test]
    fn case_insensitive_inverted_search()
    {
        let contents = "hello rust\nhello python\nhello RUST again";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.ignore_case = true;
        config.invert_match = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(2, "hello python")]
        );
    }

    #[test]
    fn case_sensitive_inverted_search()
    {
        let contents = "hello rust\nhello python\nhello RUST again";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.invert_match = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(2, "hello python"), (3, "hello RUST again")]
        );
    }

    #[test]
    fn empty_pattern_without_invert_match()
    {
        let contents = "hello\nworld";
        let config = Config::create_default_config();

        let result = find_pattern(contents, &config);
        
        assert_eq!(
            result,
            vec![(1, "hello"), (2, "world")]
        );
    }

    #[test]
    fn empty_pattern_with_invert_match()
    {
        let contents = "hello\nworld";
        let mut config = Config::create_default_config();
        config.invert_match = true;

        let result = find_pattern(contents, &config);
        
        assert_eq!(
            result,
            vec![]
        );
    }

    #[test]
    fn empty_filename()
    {
        let contents = "";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![]
        )
    }

    #[test]
    fn non_existing_pattern_without_invert_match()
    {
        let contents = "hello\nworld";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![]
        )
    }

    #[test]
    fn non_existing_pattern_with_invert_match()
    {
        let contents = "hello\nworld";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.invert_match = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(1, "hello"), (2, "world")]
        )
    }

    #[test]
    fn whole_word_search()
    {
        let contents = "rust\ntrust\nrusty\nI like rust.\ntrust rust";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.whole_word = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(1, "rust"), (4, "I like rust."), (5, "trust rust")]
        );
    }

    #[test]
    fn whole_word_with_ignore_case()
    {
        let contents = "Rust\ntrust\nI love RUST.";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.ignore_case = true;
        config.whole_word = true;

        let result = find_pattern(contents, &config);
        
        assert_eq!(
            result,
            vec![(1, "Rust"), (3, "I love RUST.")]
        ); 
    }

    #[test]
    fn whole_word_with_invert_match()
    {
        let contents = "rust\ntrust\nrusty";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.invert_match = true;
        config.whole_word = true;

        let result = find_pattern(contents, &config);
        
        assert_eq!(
            result,
            vec![(2, "trust"), (3, "rusty")]
        ); 
    }

    #[test]
    fn whole_word_empty_pattern()
    {
        let contents = "hello\nworld";
        let mut config = Config::create_default_config();
        config.whole_word = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(1, "hello"), (2, "world")]
        );
    }

    #[test]
    fn whole_word_underscore_and_digits_are_word_chars()
    {
        let contents = "rust_lang\nrust2\nrust-lang";
        let mut config = Config::create_default_config();
        config.pattern = String::from("rust");
        config.whole_word = true;

        let result = find_pattern(contents, &config);

        assert_eq!(
            result,
            vec![(3, "rust-lang")]
        );
    }

}
