// FROM HERE
// https://docs.rs/serde-json-schema/latest/serde_json_schema/

use std::error::Error;
// use crate::SerdeSchema;
use serde_json_schema::Schema;
use std::fs;

fn main() ->Result<(), Box<dyn Error>> {

    let schema_file = fs::read_to_string("./examples/address.schema.json")?;
    let address_schema = Schema::try_from(schema_file)?;
    println!("Hello, world! {}",address_schema);

Ok(())
}
