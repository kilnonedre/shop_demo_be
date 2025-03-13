use std::{fmt::Debug, fs::File};

use serde::de::DeserializeOwned;
use serde_json::Result;

pub fn read_json_from_file<T>(file_path: &str) -> Result<T>
where
    T: DeserializeOwned + Debug,
{
    let file = File::open(file_path).unwrap();
    let result: T = serde_json::from_reader(&file).unwrap();
    Ok(result)
}
