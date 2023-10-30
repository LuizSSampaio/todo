use rusqlite::{Connection, Result};

fn main() {
    let conn = Connection::open("todo.db").unwrap();
    conn.execute(
        "create table if not exists todo (
            id integer primary key, 
            title text not null, 
            is_done boolean not null
        )",
        [],
    )
    .unwrap();
}
