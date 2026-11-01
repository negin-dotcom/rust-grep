use std::{env, fs};

fn main()
{
    let args: Vec<String> = env::args().collect();
    let mut remaining_args: Vec<&String> = Vec::new();

    let mut invert_match = false;
    let mut show_line_numbers = false;
    let mut ignore_case = false; 
    let mut count_matching_lines = false;
    let mut whole_word = false;


    for arg in args.iter().skip(1)
    {
        match arg.as_str()
        {
            "-n" => show_line_numbers = true,
            "-i" => ignore_case = true,
            "-v" => invert_match = true,
            "-c" => count_matching_lines = true,
            "-w" => whole_word = true,
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
                                                  invert_match, whole_word);
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


fn find_pattern<'a>(pattern: &str, contents: &'a str, ignore_case: bool,
                    invert_match: bool, whole_word: bool) -> Vec<(usize, &'a str)> 
{
    let pattern_lower = if ignore_case { pattern.to_lowercase() } else { String::new() }; 
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            let matches = match (ignore_case, whole_word)
            {
                (true, true) => {
                    let line_lower = line.to_lowercase();
                    contains_whole_word(&line_lower, &pattern_lower)
                },
                (true, false) => {
                    let line_lower = line.to_lowercase();
                    line_lower.contains(&pattern_lower)
                },
                (false, true) => contains_whole_word(line, pattern),
                (false, false) => line.contains(pattern),
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

        let result = find_pattern("rust", contents, false,
                                        false, false);

        assert_eq!(
            result,
            vec![(1, "hello rust"), (3, "hello rust again")]
        );
    }

    #[test]
    fn case_insensitive_search()
    {
        let contents = "hello rust\nhello python\nhello RUST again";
        let result = find_pattern("rust", contents, true, 
                                        false, false);

        assert_eq!(
            result,
            vec![(1, "hello rust"), (3, "hello RUST again")]
        );                       
    }

    #[test]
    fn case_insensitive_inverted_search()
    {
        let contents = "hello rust\nhello python\nhello RUST again";
        let result = find_pattern("rust", contents, true, 
                                        true, false);

        assert_eq!(
            result,
            vec![(2, "hello python")]
        );
    }

    #[test]
    fn case_sensitive_inverted_search()
    {
        let contents = "hello rust\nhello python\nhello RUST again";
        let result = find_pattern("rust", contents, false, 
                                        true, false);

        assert_eq!(
            result,
            vec![(2, "hello python"), (3, "hello RUST again")]
        );
    }

    #[test]
    fn empty_pattern_without_invert_match()
    {
        let contents = "hello\nworld";
        let result = find_pattern("", contents, false,
                                        false, false);
        
        assert_eq!(
            result,
            vec![(1, "hello"), (2, "world")]
        );
    }

    #[test]
    fn empty_pattern_with_invert_match()
    {
        let contents = "hello\nworld";
        let result = find_pattern("", contents, false,
                                        true, false);
        
        assert_eq!(
            result,
            vec![]
        );
    }

    #[test]
    fn empty_filename()
    {
        let result = find_pattern("rust", "", false,
                                        false, false);

        assert_eq!(
            result,
            vec![]
        )
    }

    #[test]
    fn non_existing_pattern_without_invert_match()
    {
        let contents = "hello\nworld";
        let result = find_pattern("rust", contents, false,
                                        false, false);

        assert_eq!(
            result,
            vec![]
        )
    }

    #[test]
    fn non_existing_pattern_with_invert_match()
    {
        let contents = "hello\nworld";
        let result = find_pattern("rust", contents, false,
                                        true, false);

        assert_eq!(
            result,
            vec![(1, "hello"), (2, "world")]
        )
    }

    #[test]
    fn whole_word_search()
    {
        let contents = "rust\ntrust\nrusty\nI like rust.\ntrust rust";
        let result = find_pattern("rust", contents, false,
                                        false, true);

        assert_eq!(
            result,
            vec![(1, "rust"), (4, "I like rust."), (5, "trust rust")]
        );
    }

}