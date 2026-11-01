use rust_grep::{find_pattern, Config};



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

