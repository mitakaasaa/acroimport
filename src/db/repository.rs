use crate::models::form::FormData;
use rusqlite::{Connection, Result, params};

/// Saves form in DB with unique constraint check.
///
/// Logic for each entity:
///   1. Try to insert via INSERT OR IGNORE
///   2. Get id — row we've inserted right now,
///      or already existing one (via SELECT on business key)
pub fn save_form(conn: &Connection, form: &FormData) -> Result<()> {
    // ── 1. Owner ──────────────────────────────────────────────────────────────
    conn.execute(
        "INSERT OR IGNORE INTO owners (first_name, last_name) VALUES (?1, ?2)",
        params![form.first_name, form.last_name],
    )?;

    let owner_id: i64 = conn.query_row(
        "SELECT id FROM owners WHERE first_name = ?1 AND last_name = ?2",
        params![form.first_name, form.last_name],
        |row: &rusqlite::Row<'_>| row.get(0),
    )?;

    // ── 2. Dog ────────────────────────────────────────────────────────────────
    // UNIQUE: name + breed + owner.
    // Same owner with 2 dogs → 2 unique rows in dogs.
    conn.execute(
        "INSERT OR IGNORE INTO dogs (name, breed, owner_id) VALUES (?1, ?2, ?3)",
        params![form.dog_name, form.breed, owner_id],
    )?;

    let dog_id: i64 = conn.query_row(
        "SELECT id FROM dogs WHERE name = ?1 AND breed = ?2 AND owner_id = ?3",
        params![form.dog_name, form.breed, owner_id],
        |row: &rusqlite::Row<'_>| row.get(0),
    )?;

    // ── 3. Enrollments ────────────────────────────────────────────────────────
    // UNIQUE (dog_id, course_id) on table + OR IGNORE = no duplicated values
    for course in &form.courses {
        let course_id: i64 = conn.query_row(
            "SELECT id FROM courses WHERE name = ?1",
            params![course.display_name()],
            |row: &rusqlite::Row<'_>| row.get(0),
        )?;

        conn.execute(
            "INSERT OR IGNORE INTO enrollments (dog_id, course_id) VALUES (?1, ?2)",
            params![dog_id, course_id],
        )?;
    }

    Ok(())
}

/// Returns members of given course: (first_name, last_name, dog_name, breed)
pub fn get_enrollments_for_course(
    conn: &Connection,
    course_name: &str,
) -> Result<Vec<(String, String, String, String)>> {
    let mut stmt: rusqlite::Statement<'_> = conn.prepare(
        "
        SELECT
            o.first_name,
            o.last_name,
            d.name  AS dog_name,
            d.breed
        FROM enrollments e
        JOIN dogs    d ON d.id = e.dog_id
        JOIN owners  o ON o.id = d.owner_id
        JOIN courses c ON c.id = e.course_id
        WHERE c.name = ?1
        ORDER BY o.last_name, o.first_name
    ",
    )?;

    let rows = stmt.query_map(params![course_name], |row: &rusqlite::Row<'_>| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;

    rows.collect()
}
