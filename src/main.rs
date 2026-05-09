mod db;
mod importer;
mod models;
mod pdf;

use crate::importer::import::run_import;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Fehlerhafter Programmaufruf!");
        println!("Verwendung: hundeschule.exe /n | /imp | /exp");
        return Ok(());
    }

    match args[1].as_str() {
        "/n" => println!("Denys Stamat - Kate Lagutin - Malcolm Apaloo"),
        "/imp" => run_import()?,
        "/exp" => println!("Exportiert!"), // TODO
        _ => {
            println!("Fehlerhafter Programmaufruf!");
            println!("Verwendung: hundeschule.exe /n | /imp | /exp");
        }
    }

    Ok(())
}
