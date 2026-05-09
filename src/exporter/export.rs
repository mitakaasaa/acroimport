use csv::WriterBuilder;
use std::path::Path;

use crate::db::{connection, repository};
use crate::models::course::Course;

const ALL_COURSES: &[Course] = &[
    Course::Welpenkurs,
    Course::Junghundekurs,
    Course::Agility,
    Course::Obedience,
    Course::Mantrailing,
];

pub fn run_export() -> anyhow::Result<()> {
    let conn: rusqlite::Connection = connection::open()?;

    for course in ALL_COURSES {
        let name: &str = course.display_name();
        let rows: Vec<(String, String, String, String)> =
            repository::get_enrollments_for_course(&conn, name)?;

        if rows.is_empty() {
            println!(
                "Teilnahmeliste_{}.csv – keine Einträge vorhanden, Datei wird nicht erstellt",
                name
            );
            continue;
        }

        let filename: String = format!("Teilnahmeliste_{}.csv", name);
        write_csv(&filename, &rows)?;
        println!("{} erfolgreich erstellt", filename);
    }

    Ok(())
}

/// Writes rows in CSV file in current directory.
/// Columns: Vorname, Nachname, Hundename, Rasse
fn write_csv(filename: &str, rows: &[(String, String, String, String)]) -> anyhow::Result<()> {
    let path: &Path = Path::new(filename);
    let mut wtr: csv::Writer<std::fs::File> =
        WriterBuilder::new().delimiter(b',').from_path(path)?;

    // Header
    wtr.write_record(["Vorname", "Nachname", "Hundename", "Rasse"])?;

    for (first_name, last_name, dog_name, breed) in rows {
        wtr.write_record([first_name, last_name, dog_name, breed])?;
    }

    wtr.flush()?;
    Ok(())
}
