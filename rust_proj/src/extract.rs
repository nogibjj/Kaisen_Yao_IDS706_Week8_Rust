use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;

pub fn extract(url: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Set default values
    let url = if url.is_empty() {
        "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/births/US_births_2000-2014_SSA.csv"
    } else {
        url
    };
    let file_path = if file_path.is_empty() {
        "../data/US_births.csv"
    } else {
        file_path
    };

    // Send HTTP GET request and get content
    let response = get(url)?;
    let content = response.bytes()?;

    // Write to file
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;

    Ok(file_path.to_string())
}
