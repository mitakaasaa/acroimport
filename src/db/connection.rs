use rusqlite::{Connection, Result};

/// Opens (or creates) file hundeschule.db in current directory
/// and init DB schema in 3NF.
pub fn open() -> Result<Connection> {
    let conn: Connection = Connection::open("hundeschule.db")?;
    initialise_schema(&conn)?;
    Ok(conn)
}

/// Creates all tables if not present.
///
/// Schema (3NF):
///   owners      (id, first_name, last_name)
///   dogs        (id, name, breed, owner_id → owners.id)
///   courses     (id, name)               ← List
///   enrollments (id, dog_id, course_id)  ← M:N
fn initialise_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
 
        CREATE TABLE IF NOT EXISTS owners (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name  TEXT NOT NULL,
            UNIQUE (first_name, last_name)
        );
 
        CREATE TABLE IF NOT EXISTS dogs (
            id       INTEGER PRIMARY KEY AUTOINCREMENT,
            name     TEXT NOT NULL,
            breed    TEXT NOT NULL,
            owner_id INTEGER NOT NULL REFERENCES owners(id),
            UNIQUE (name, breed, owner_id)
        );
 
        CREATE TABLE IF NOT EXISTS courses (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );
 
        CREATE TABLE IF NOT EXISTS enrollments (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            dog_id    INTEGER NOT NULL REFERENCES dogs(id),
            course_id INTEGER NOT NULL REFERENCES courses(id),
            UNIQUE (dog_id, course_id)
        );
    ",
    )?;

    conn.execute_batch(
        "
        INSERT OR IGNORE INTO courses (name) VALUES ('Welpenkurs');
        INSERT OR IGNORE INTO courses (name) VALUES ('Junghundekurs');
        INSERT OR IGNORE INTO courses (name) VALUES ('Agility');
        INSERT OR IGNORE INTO courses (name) VALUES ('Obedience');
        INSERT OR IGNORE INTO courses (name) VALUES ('Mantrailing');
    ",
    )?;

    Ok(())
}
