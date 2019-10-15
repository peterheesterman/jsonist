extern crate jsonist;

use std::fs;

use jsonist:: { lint, FormatConfig, Delimiter, DelimiterCount };

#[test]
fn integration_test_max_depth_one() {
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

    let expected_out_file_path = "./tests/output/con_parse_correct_json.json";

    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    match lint(json, None) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

fn complex_json() -> String {
    r#"
{
	"destination_addresses": [
		"Washington, DC, USA",
		"Philadelphia, PA, USA",
		"Santa Barbara, CA, USA",
		"Miami, FL, USA",
		"Austin, TX, USA",
		"Napa County, CA, USA"
	],
	"origin_addresses": [ "New York, NY, USA" ],
	"rows": [{
		"elements": [{
                                "distance": { "text": "227 mi", "value": 365468 },
				"duration": { "text": "3 hours 54 mins", "value": 14064 },
				"status": "OK"
			},
			{
				"distance": { "text": "94.6 mi", "value": 152193 },
				"duration": { "text": "1 hour 44 mins", "value": 6227 },
				"status": "OK"
			},
			{
				"distance": { "text": "2,878 mi", "value": 4632197 },
				"duration": { "text": "1 day 18 hours", "value": 151772 },
				"status": "OK"
			},
			{
				"distance": { "text": "1,286 mi", "value": 2069031 },
				"duration": { "text": "18 hours 43 mins", "value": 67405 },
				"status": "OK"
			},
			{
				"distance": { "text": "1,742 mi", "value": 2802972 },
				"duration": { "text": "1 day 2 hours", "value": 93070 },
				"status": "OK"
			},
			{
				"distance": { "text": "2,871 mi", "value": 4620514 },
				"duration": { "text": "1 day 18 hours", "value": 152913 },
				"status": "OK"
			}
		]
	}],
	"status": "OK"
}
    "#.to_owned()
}

#[test]
fn integration_test_more_depth() {
    let json = complex_json();

    let expected_out_file_path = "./tests/output/con_parse_complex_json_four_spaces.json";
    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    match lint(json, None) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn integration_test_more_depth_two_spaces() {
    let json = complex_json();

    let expected_out_file_path = "./tests/output/con_parse_complex_json_two_spaces.json";
    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    let config = FormatConfig::new(Delimiter::Spaces(DelimiterCount::Two));

    match lint(json, Some(config)) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn integration_test_more_depth_tabs() {
    let json = complex_json();

    let expected_out_file_path = "./tests/output/con_parse_complex_json_tabs.json";
    let expected_contents = fs::read_to_string(expected_out_file_path).expect("No output file");

    let config = FormatConfig::new(Delimiter::Tabs);

    match lint(json, Some(config)) {
        Ok(value) => assert_eq!(value, expected_contents),
        Err(e) => panic!("{}", e)
    }
}

