use std::error::Error;
use serde_json::Value;

mod utils {
    pub mod io {
        pub mod csv_io;
        pub mod parquet_io;
        pub mod json_io;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let x: &str = "hello";
    println!("print '{x}'");

    let df = utils::io::csv_io::read_csv_to_dataframe("./data/data.csv")?;
    println!("{:?}", df);

    utils::io::csv_io::create_csv("./data/created_data.csv")?;

    let df = utils::io::parquet_io::read_parquet_to_dataframe("./data/data.parquet")?;
    println!("{:?}", df);
    
    utils::io::parquet_io::create_parquet("./data/created_data.parquet")?;
    
    let df = utils::io::json_io::read_to_dataframe("./data/data.json");
    println!("{:?}", df);

    utils::io::json_io::create_data("./data/created_data.json")?;

    let json_output = utils::io::json_io::read_to_json("./data/data.json")?;
    
    // Access a specific key
    let got_json_val = json_output.get("integer_key");
    let extracted_to_string = extract_string(got_json_val);
        // Assign the result of the match expression to a variable
    let converted_to_string = match extracted_to_string {
        Some(s) => s,
        None => String::from("No value found"),
    };
    println!("converted: '{converted_to_string}'");

    if let Some(value) = json_output.get("string_key") {
        println!("Value: {}", value);
    } else {
        println!("Key not found");
    }

    Ok(())
}

fn extract_string(value: Option<&Value>) -> Option<String> {
    value.and_then(|v| v.as_str().map(|s| s.to_string()))
}