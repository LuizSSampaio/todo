use rusqlite::{Connection, Result};

const DB_PATH: &str = "todo.db";
const DB_TABLE_NAME: &str = "todo";

#[derive(Debug)]
struct ToDo {
    name: String,
    is_done: bool,
}

pub fn connect() -> rusqlite::Connection {
    let connection = Connection::open(DB_PATH).unwrap();

    let table_exists: bool = connection
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='(table_name)'",
            &[DB_TABLE_NAME],
            |row| row.get(0),
        )
        .unwrap();

    if !table_exists {
        connection
            .execute(
                "CREATE TABLE (table_name) (id INTEGER PRIMARY KEY, name TEXT, is_done BOOLEAN)",
                &[DB_TABLE_NAME],
            )
            .unwrap();
    }

    connection
}

pub fn create_new_list(db: rusqlite::Connection) {
    db.execute("DROP TABLE IF EXISTS (table_name)", &[DB_TABLE_NAME])
        .unwrap();
    db.execute(
        "CREATE TABLE (table_name) (id INTEGER PRIMARY KEY, name TEXT, is_done BOOLEAN)",
        &[DB_TABLE_NAME],
    )
    .unwrap();
}

pub fn add_list_item() {}

pub fn remove_list_item() {}

pub fn done_list_item() {}

pub fn undone_list_item() {}
