extern crate tinyjson;

use tinyjson::*;

#[test]
fn it_works() {
    {
        let s = r#"
          {
            "bool": true,
            "arr": [1, null, "test"],
            "nested": {
              "blah": false,
              "blahblah": 3.14
            },
            "unicode": "\u2764"
          }
        "#;
        let mut p = make_str_parser(s);
        let parsed = p.parse();
        assert!(parsed.is_ok(), "Failed to parse: {:?}", parsed);
    }
}
