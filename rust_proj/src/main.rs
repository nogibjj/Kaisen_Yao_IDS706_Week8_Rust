use std::fs::File;
use std::io::Write;
use std::time::Instant;
use systemstat::{Platform, System};

mod extract;
mod query;
mod transform_load;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sys = System::new();

    // Extract
    println!("Extracting data...");
    extract::extract("", "")?;

    let start_time = Instant::now();
    let memory_before = sys.memory()?.free.as_u64() as f64 / (1024.0 * 1024.0);

    // Transform and load
    println!("Transforming data...");
    transform_load::load("")?;

    // Query
    println!("Querying data...");
    query::query()?;

    let elapsed_time_micros = start_time.elapsed().as_micros();
    let memory_after = sys.memory()?.free.as_u64() as f64 / (1024.0 * 1024.0);
    let memory_used = memory_before - memory_after;

    // Write performance metrics to file
    let mut file = File::create("../rust_performance.md")?;
    writeln!(
        file,
        "Elapsed time: {:.2} microseconds\nMemory used: {:.2} MB",
        elapsed_time_micros,
        memory_used.abs() // Use absolute value, as memory calculation may be negative
    )?;

    Ok(())
}
