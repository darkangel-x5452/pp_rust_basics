use polars::prelude::*;
// use polars_core::prelude::*;
use reqwest::{Client, Error};
use serde_json::json;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();

    // Define the JSON payload
    let payload = json!({
      "query": "query GetDeveloperApp($id: ID!) {\n  app: developerApp(id: $id) {\n    id\n    automount\n  }\n}",
      "variables": {
        "id": "app_ONUCvEav"
      },
      "operationName": "GetDeveloperApp"
    });

    // Send a GET request to the specified URL
    // let response = reqwest::get("https://api.propps.com/v1/debug/").await?;
    let response = client
        .post("https://services.propps.com/graphql")
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let resp_str = response.text().await?;
        println!("{:#?}", resp_str);

        
        let json: serde_json::Value =
            serde_json::from_str(&resp_str).expect("JSON was not well-formatted");
        let json_str = json.to_string();
        println!("{json_str}");
        
        // let resp_json: serde_json::Value = response.json().await?;
        // println!("{:#?}", resp_str);

        // Create a Cursor from the JSON string
        let file = Cursor::new(json_str);
        // Read the JSON data into a DataFrame
        let df = JsonReader::new(file).finish();
            // .with_json_format(JsonFormat::JsonLines)
            // .infer_schema_len(NonZeroUsize::new(3))
            // .with_batch_size(NonZeroUsize::new(3).unwrap())
            // .finish();

        // Print the DataFrame
        println!("{:?}", df);
    } else {
        eprintln!("Request failed with status: {}", response.status());
    }

    // Check if the response status is 200 OK
    // if response.status() != StatusCode::OK {
    //     // Return an error if the status is not 200
    //     return Err(reqwest::Error::new(
    //         reqwest::ErrorKind::Status,
    //         format!("Request failed with status: {}", response.status()),
    //     ));
    // }

    // Parse the response body as JSON
    // let json: Value = response.json().await?;

    // // Print the entire JSON response
    // println!("Full JSON response:\n{:#}", json);

    // // If the JSON is an object, iterate over its key-value pairs
    // if let Value::Object(map) = json {
    //     for (key, value) in map {
    //         println!("Key: {}, Value: {}", key, value);
    //     }
    // }

    Ok(())
}
