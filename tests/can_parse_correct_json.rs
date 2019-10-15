extern crate jsonist;

use std::fs;

use jsonist::lint;

#[test]
fn integration_test() {
    // TODO: depth and comma placement still need addressing in the output of this test
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

    let expected_out_file_path = "./tests/output/con_parse_corrrect_json.json";

    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    match lint(json) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}
