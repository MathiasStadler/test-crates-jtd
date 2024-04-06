// FROM HERE
// https://docs.rs/jtd/latest/jtd/

#[allow(unused_imports)] 
use jtd::{Schema, ValidationErrorIndicator};
#[allow(unused_imports)]
use serde_json::json;

#[cfg(test)]
mod tests {

    #[test]
    fn simple() {
        let schema = Schema::from_serde_schema(
            serde_json::from_value(json!({
                "properties": {
                    "name": { "type": "string" },
                    "age": { "type": "uint32" },
                    "phones": {
                        "elements": {
                            "type": "string"
                        }
                    }
                }
            }))
            .expect("Parse schema"),
        )
        .expect("Construct schema from JSON data");

        schema.validate().expect("Invalid schema");

        // Since this first example is valid, we'll get back an empty list of
        // validation errors.
        let input_ok = json!({
            "name": "John Doe",
            "age": 43,
            "phones": ["+44 1234567", "+44 2345678"]
        });

        assert_eq!(
            Vec::<ValidationErrorIndicator>::new(),
            jtd::validate(&schema, &input_ok, Default::default()).unwrap(),
        );

        // This example is invalid, so we'll get back three validation errors:
        //
        // 1. "name" is required but not present,
        // 2. "age" has the wrong type
        // 3. "phones[1]" has the wrong type
        let input_bad = json!({
            "age": "43",
            "phones": ["+44 1234567", 442345678]
        });

        // Each error indicator has two pieces of information: the path to the part
        // of the input that was rejected (the "instance path"), and the part of the
        // schema that rejected it (the "schema path").
        //
        // The exact values of the instance path and schema path is specified in the
        // JSON Type Definition spec.
        assert_eq!(
            vec![
                // "age" has the wrong type (required by "/properties/age/type")
                ValidationErrorIndicator {
                    instance_path: vec!["age".into()],
                    schema_path: vec!["properties".into(), "age".into(), "type".into()],
                },
                // "name" is missing (required by "/properties/name")
                ValidationErrorIndicator {
                    instance_path: vec![],
                    schema_path: vec!["properties".into(), "name".into()],
                },
                // "phones/1" has the wrong type (required by "/properties/phones/elements/type")
                ValidationErrorIndicator {
                    instance_path: vec!["phones".into(), "1".into()],
                    schema_path: vec![
                        "properties".into(),
                        "phones".into(),
                        "elements".into(),
                        "type".into()
                    ],
                },
            ],
            jtd::validate(&schema, &input_bad, Default::default()).unwrap(),
        );
    }
}
