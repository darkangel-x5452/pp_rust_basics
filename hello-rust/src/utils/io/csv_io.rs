use polars::prelude::*;
use std::error::Error;
// use std::fs::File;

/// Reads a CSV file from the given file path into a Polars `DataFrame`.
///
/// # Arguments
///
/// * `file_path` – A string slice holding the path to the CSV file to read.
///
/// # Returns
///
/// On success, returns `Ok(DataFrame)` containing the parsed CSV data;
/// on failure, returns `Err(Box<dyn Error>)` with the underlying I/O or parse error.
///
/// # Errors
///
/// This function will return an error if:
/// - The file at `file_path` cannot be opened.
/// - The CSV reader cannot parse the file.
/// - Finalizing the `DataFrame` fails.
///
/// # Examples
///
/// ```no_run
/// # use std::error::Error;
/// # use polars::prelude::*;
/// # use your_crate::read_csv_to_dataframe;
/// let df = read_csv_to_dataframe("data.csv")?;
/// assert!(df.height() > 0);
/// # Ok::<(), Box<dyn Error>>(())
/// ```
pub fn read_csv_to_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let mut df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(file_path.into()))
        .unwrap()
        .finish()
        .unwrap();
    let mut file = std::fs::File::create("./data/data.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    Ok(df)
}

pub fn create_csv(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let mut df = df!(
        "foo" => &[1, 2, 3],
        "bar" => &[None, Some("bak"), Some("baz")],
    )
    .unwrap();
    
    let mut file = std::fs::File::create(file_path).unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
    
    Ok(df)
}
