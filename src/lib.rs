use std::fs;

const COLORS: [&str; 2] = ["\x1b[33m", "\x1b[36m"];

pub struct Config
{
    pub pattern: String,
    pub filenames: Vec<String>,
    pub ignore_case: bool, 
    pub invert_match: bool,
    pub whole_word: bool,
    pub show_line_numbers: bool,
    pub count_matching_lines: bool,
}

impl Config 
{
    pub fn create_default_config() -> Config
    {
        Config {
            pattern: String::new(),
            filenames: Vec::new(),
            ignore_case: false,
            invert_match: false,
            whole_word: false,
            show_line_numbers: false,
            count_matching_lines: false,
        }
    }
}

pub fn is_word_char(c: char) -> bool 
{
    c.is_alphanumeric() || c == '_'
}

pub fn contains_whole_word(line: &str, pattern: &str) -> bool 
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
                            .is_none_or( |c| !is_word_char(c));
        
        let after_ok = line[end..]
                            .chars()
                            .next()
                            .is_none_or(|c| !is_word_char(c));
        
        if before_ok && after_ok 
        {
            return true;
        }
    }    
    false       
}


pub fn find_pattern<'a>(contents: &'a str, config: &Config) -> Vec<(usize, &'a str)> 
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


pub fn search_contents(
    filename: &str, 
    contents: &str,
    config: &Config,
    show_filename: bool
) -> Vec<String> 
{
    let result = find_pattern(contents, config);

    if config.count_matching_lines 
    {
        if show_filename 
        {
            return vec![format!("{}{}\x1b[0m:{}", color_for(filename), filename, result.len())];

        } else {
            return vec![result.len().to_string()];
        }
    }

    let mut output = Vec::new();

    for (idx, line) in result.iter() 
    {
        let mut prefix = String::new();

        if show_filename 
        {
            prefix.push_str(color_for(filename));
            prefix.push_str(filename);
            prefix.push_str("\x1b[0m");
            prefix.push_str(" -> ");
        } 

        if config.show_line_numbers 
        {
            prefix.push_str(&format!("{}: ", idx));
        }

        output.push(format!("{}{}", prefix, line));
    }

    output
}


pub fn search_one_file(filename: &str, config: &Config, show_filename: bool) 
{
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Could not read file {}: {}", filename, error);
            return;
        }
    };

    println!();

    for line in search_contents(filename, &contents, config, show_filename)
    {
        println!("{}", line);
    }
}

pub fn color_for(filename: &str) -> &'static str 
{
    let sum: usize = filename.bytes().map(|b| b as usize).sum();
    COLORS[sum % COLORS.len()]
}