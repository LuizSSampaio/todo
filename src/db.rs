use rusqlite::{Connection, Result, NO_PARAMS};
use std::path::Path;

const DB_PATH: String = "todo.db";
const DB_TABLE_NAME: String = "todo";

#[derive(Debug)]
struct To_Do {
    name: String,
    is_done: bool,
}

fn connect() -> rusqlite::Connection {
    let connection = Connection::open(DB_PATH)?;

    let table_exists: bool = connection.query_row(
        &format!(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='{}'",
            DB_TABLE_NAME
        ),
        NO_PARAMS,
        |row| row.get(0),
    )?;

    if !table_exists {
        connection.execute(
            &format!(
                "CREATE TABLE {} (id INTEGER PRIMARY KEY, name TEXT, is_done BOOLEAN)",
                DB_TABLE_NAME
            ),
            NO_PARAMS,
        )?;
    }

    connection
}

fn create_new_list() {}

fn add_list_item() {}

fn remove_list_item() {}

fn done_list_item() {}

fn undone_list_item() {}
