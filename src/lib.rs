use std::fs;

const COLORS: [&str; 2] = ["\x1b[33m", "\x1b[36m"];   // yellow, cyan

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub struct Config
{
    pub pattern: String,
    pub filenames: Vec<String>,
    pub ignore_case: bool, 
    pub invert_match: bool,
    pub whole_word: bool,
    pub show_line_numbers: bool,
    pub count_matching_lines: bool,
    pub count_matches: bool,
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
            count_matches: false,
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
    let pattern_lower = config.ignore_case
                                    .then(|| config.pattern.to_lowercase())
                                    .unwrap_or_default();

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

    if config.count_matching_lines && show_filename 
    {
        return vec![format!("{}{}{}:{}", color_for(filename), filename, RESET, result.len())];

    } else if config.count_matching_lines && !show_filename {
        return vec![result.len().to_string()];
    }

    let mut output = Vec::new();

    for (idx, line) in result.iter() 
    {
        let mut prefix = String::new();

        if show_filename 
        {
            prefix.push_str(color_for(filename));
            prefix.push_str(filename);
            prefix.push_str(RESET);
            prefix.push_str(" -> ");
        } 

        if config.show_line_numbers 
        {
            prefix.push_str(&format!("{}: ", idx));
        }

        output.push(format!("{}{}", prefix, line));
    }

    if config.count_matches 
    {
        output.push(count_matches_with_color(total_occurrences(&result, config)));
    }

    output
}

pub fn count_matches_with_color(count: usize) -> String
{
    if count == 0 {
        format!("{}No matches found.{}", RED, RESET)
    
    } else {
        format!("{}{} match(es) found.{}", GREEN, count, RESET)
    }
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

    search_contents(filename, &contents, config, show_filename).into_iter()
                                                               .for_each(|line| println!("{}", line));
}

pub fn color_for(filename: &str) -> &'static str 
{
    let sum: usize = filename.bytes().map(|b| b as usize).sum();
    COLORS[sum % COLORS.len()]
}

pub fn count_in_line(line: &str, pattern: &str, whole_word: bool) -> usize 
{
    if pattern.is_empty()
    {
        return 0;
    }

    line.match_indices(pattern)
        .filter(|(start, _)| {
            if !whole_word 
            {
                return true;
            }
            let end = start + pattern.len();
            let before_ok = line[..*start].chars().next_back().is_none_or(|c| !is_word_char(c));
            let after_ok = line[end..].chars().next().is_none_or(|c| !is_word_char(c));
            before_ok && after_ok
        })
        .count()
}


pub fn total_occurrences(result: &[(usize, &str)], config: &Config) -> usize 
{
    let pattern = if config.ignore_case {
        config.pattern.to_lowercase()

    } else {
        config.pattern.clone()
    };

    result.iter()
          .map(|(_, line)| {
                if config.ignore_case 
                {
                    count_in_line(&line.to_lowercase(), &pattern, config.whole_word)
                
                } else {
                    count_in_line(line, &pattern, config.whole_word)
                }
          })
          .sum()
}