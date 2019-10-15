extern crate jsonist;

use std::fs;

use jsonist:: { format, FormatConfig, Delimiter, DelimiterCount, FormatterError };

#[test]
fn integration_can_use_errors_in_public_interface() {
    let json = r#"
        {
            "name": "Peter",
            "name": 2,
        }
    "#.to_owned();

    match format(json, None) {
        Ok(value) => assert_eq!(value, String::from("")),
        Err(e) => { 
            match e {
                FormatterError::DuplicateKeyEntry(key) => {
                    assert_eq!(key, "name");
                },
                _ => panic!("Oh no")
            }
        }
    }
}

#[test]
fn integration_test_max_depth_one() {
    let json = r#"
        {
            "name": "Peter",
            "leg_count": 2,
            "languages": ["rust", "javascript", "lisp"],
            "address": {
                "street_name": "lets not put this online",
                "city": "a large one"
            },
            "winner": true
        }
    "#.to_owned();

    let expected_out_file_path = "./tests/output/con_parse_correct_json.json";

    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    match format(json, None) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

fn complex_json(file_name: &'static str) -> String {
    fs::read_to_string(file_name).expect("failed to open file in test")
}

#[test]
fn integration_test_more_depth() {
    let json = complex_json("./tests/input/sample.json");

    let expected_out_file_path = "./tests/output/con_parse_complex_json_four_spaces.json";
    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    match format(json, None) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn integration_test_more_depth_two_spaces() {
    let json = complex_json("./tests/input/sample.json");

    let expected_out_file_path = "./tests/output/con_parse_complex_json_two_spaces.json";
    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    let config = FormatConfig::new(Delimiter::Spaces(DelimiterCount::Two));

    match format(json, Some(config)) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn integration_test_more_depth_tabs() {
    let json = complex_json("./tests/input/sample.json");

    let expected_out_file_path = "./tests/output/con_parse_complex_json_tabs.json";
    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    let config = FormatConfig::new(Delimiter::Tabs);

    match format(json, Some(config)) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

