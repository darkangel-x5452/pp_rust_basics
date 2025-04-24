use polars::prelude::*;
use std::error::Error;
use std::fs;

pub fn read_to_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    use polars::prelude::*;

    let mut file = std::fs::File::open(file_path).unwrap();
    let df = JsonReader::new(&mut file).finish()?;
    Ok(df)
}

pub fn create_data(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let mut df = df!(
        "foo" => &[1, 2, 3],
        "bar" => &[None, Some("bak"), Some("baz")],
    )
    .unwrap();

    let mut file = std::fs::File::create(file_path).unwrap();

    // json
    JsonWriter::new(&mut file)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df)
        .unwrap();

    // ndjson
    // JsonWriter::new(&mut file)
    //     .with_json_format(JsonFormat::JsonLines)
    //     .finish(&mut df)
    //     .unwrap();

    Ok(df)
}

pub fn read_to_json(file_path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    let json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    Ok(json)
}
