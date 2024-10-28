use csv::Reader;
use rusqlite::{Connection, Result};
use std::error::Error;
use std::fmt;
use std::path::Path;

// Define custom error type
#[derive(Debug)]
pub enum LoadError {
    SqliteError(rusqlite::Error),
    CsvError(csv::Error),
}

// Implement error type conversion
impl From<rusqlite::Error> for LoadError {
    fn from(err: rusqlite::Error) -> LoadError {
        LoadError::SqliteError(err)
    }
}

impl From<csv::Error> for LoadError {
    fn from(err: csv::Error) -> LoadError {
        LoadError::CsvError(err)
    }
}

// Implement Display trait
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::SqliteError(e) => write!(f, "SQLite error: {}", e),
            LoadError::CsvError(e) => write!(f, "CSV error: {}", e),
        }
    }
}

// Implement Error trait
impl Error for LoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoadError::SqliteError(e) => Some(e),
            LoadError::CsvError(e) => Some(e),
        }
    }
}

// Modify function signature to use new error type
pub fn load(dataset: &str) -> Result<String, LoadError> {
    // Set default dataset path
    let dataset = if dataset.is_empty() {
        "../data/US_births.csv"
    } else {
        dataset
    };

    // Create database connection - add mut keyword
    let mut conn = Connection::open("../data/US_births_DB.db")?;

    // Add debug print before batch execution
    println!("Executing schema setup...");
    conn.execute_batch(
        "
        DROP TABLE IF EXISTS US_births_DB;
        
        CREATE TABLE US_births_DB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year INTEGER,
            month INTEGER,
            date_of_month INTEGER,
            day_of_week INTEGER,
            births INTEGER
        );
    ",
    )?;

    // Add debug print for CSV reading
    println!("Reading CSV file: {}", dataset);
    let mut rdr = Reader::from_path(Path::new(dataset))?; // Directly use ? operator, no need for map_err

    // Prepare INSERT statement
    let tx = conn.transaction()?;
    {
        println!("Preparing insert statement...");
        let mut stmt = tx.prepare(
            "INSERT INTO US_births_DB (
                year, month, date_of_month, day_of_week, births
            ) 
            VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        println!("Starting to insert records...");
        for (i, result) in rdr.records().enumerate() {
            let record = result?;
            match stmt.execute([
                record.get(0).unwrap_or_default(), // year
                record.get(1).unwrap_or_default(), // month
                record.get(2).unwrap_or_default(), // date_of_month
                record.get(3).unwrap_or_default(), // day_of_week
                record.get(4).unwrap_or_default(), // births
            ]) {
                Ok(_) => {
                    if i % 1000 == 0 {
                        println!("Inserted {} records", i);
                    }
                }
                Err(e) => println!("Error inserting record {}: {}", i, e),
            }
        }
    }

    // Commit transaction
    tx.commit()?;

    Ok("../data/US_births_DB.db".to_string())
}
