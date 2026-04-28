mod models;
mod pdf;

use models::form::FormData;
use pdf::parser::parse_pdf;

fn main() -> anyhow::Result<()> {
    let file_path: &str = "test.pdf";

    println!("📄 Reading file: {}\n", file_path);

    let raw_data: std::collections::HashMap<String, String> = parse_pdf(file_path)?;

    println!("--- RAW DATA ---");
    for (k, v) in &raw_data {
        println!("{} = {}", k, v);
    }

    println!("\n--- PARSED FORM ---");

    match FormData::try_from(&raw_data) {
        Ok(form) => {
            println!("{:#?}", form);
            println!("\n✅ Form is valid!");
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    Ok(())
}
