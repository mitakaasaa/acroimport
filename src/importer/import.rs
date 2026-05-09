use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

use crate::db::connection;
use crate::db::repository;
use crate::models::form::FormData;
use crate::pdf::parser::parse_pdf;

pub fn run_import() -> anyhow::Result<()> {
    let conn: rusqlite::Connection = connection::open()?;

    let current_dir: PathBuf = std::env::current_dir()?;
    let imported_dir: PathBuf = current_dir.join("importiert");

    if !imported_dir.exists() {
        fs::create_dir(&imported_dir)?;
    }

    for entry in fs::read_dir(&current_dir)? {
        let entry: fs::DirEntry = entry?;
        let path: PathBuf = entry.path();

        if is_pdf(&path) {
            process_file(&path, &imported_dir, &conn)?;
        }
    }

    Ok(())
}

fn is_pdf(path: &Path) -> bool {
    path.extension()
        .and_then(|ext: &std::ffi::OsStr| ext.to_str())
        .map(|ext: &str| ext.eq_ignore_ascii_case("pdf"))
        .unwrap_or(false)
}

fn process_file(
    path: &Path,
    imported_dir: &Path,
    conn: &rusqlite::Connection,
) -> anyhow::Result<()> {
    let filename: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();

    let raw: std::collections::HashMap<String, String> = match parse_pdf(path.to_str().unwrap()) {
        Ok(data) => data,
        Err(_) => {
            println!("{} nicht erfolgreich importiert!", filename);
            return Ok(());
        }
    };

    let form: FormData = match FormData::try_from(&raw) {
        Ok(f) => f,
        Err(_) => {
            println!("{} nicht erfolgreich importiert!", filename);
            return Ok(());
        }
    };

    match repository::save_form(conn, &form) {
        Ok(_) => {
            println!("{} erfolgreich importiert!", filename);
            move_file(path, imported_dir)?;
        }
        Err(e) => {
            println!(
                "{} nicht erfolgreich importiert! (DB-Fehler: {})",
                filename, e
            );
        }
    }

    Ok(())
}

fn move_file(path: &Path, target_dir: &Path) -> anyhow::Result<()> {
    let filename: &std::ffi::OsStr = path.file_name().unwrap();
    let target_path: PathBuf = target_dir.join(filename);
    fs::rename(path, target_path)?;
    Ok(())
}
