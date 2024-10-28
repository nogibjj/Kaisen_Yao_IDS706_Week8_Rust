use rusqlite::{Connection, Result};

pub fn query() -> Result<String> {
    // Connect to database
    let conn = Connection::open("../data/US_births_DB.db")?;

    // Execute SELECT
    let mut stmt = conn.prepare("SELECT * FROM US_births_DB")?;
    let _rows = stmt.query([])?;
    // If you need to process the results, you can iterate over rows

    // Execute INSERT
    conn.execute(
        "INSERT INTO US_births_DB (year, month, date_of_month, day_of_week, births) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        [2008, 8, 8, 1, 9999],
    )?;

    // Execute UPDATE
    conn.execute(
        "UPDATE US_births_DB 
         SET day_of_week = ?1, births = ?2
         WHERE id = ?3",
        [1, 6666, 1],
    )?;

    // Execute DELETE
    conn.execute(
        "DELETE FROM US_births_DB 
         WHERE id = ?1",
        [2],
    )?;

    // Commit changes will be automatically executed when the Connection is dropped
    Ok("Success".to_string())
}
