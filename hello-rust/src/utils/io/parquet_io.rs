use polars::prelude::*;
use std::error::Error;

pub fn read_parquet_to_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let mut file = std::fs::File::open(file_path).unwrap();

    let df = ParquetReader::new(&mut file).finish().unwrap();
    Ok(df)
}

pub fn create_parquet(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    let mut df = df!(
        "foo" => &[1, 2, 3],
        "bar" => &[None, Some("bak"), Some("baz")],
    )
    .unwrap();
    
    let mut file = std::fs::File::create(file_path).unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
    
    Ok(df)
}
