mod pdf;
use pdf::parser::parse_pdf;

fn main() -> anyhow::Result<()> {
    let data = parse_pdf("test.pdf")?;

    println!("--- RESULT ---");
    for (k, v) in data {
        println!("{} = {}", k, v);
    }

    Ok(())
}
