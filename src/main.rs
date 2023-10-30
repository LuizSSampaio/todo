use args::{ActionType, TodoArgs};
use clap::Parser;
use rusqlite::{Connection, Result};

mod args;

fn main() {
    let args = TodoArgs::parse();

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

    match args.action_type {
        ActionType::Add(add_args) => {
            conn.execute(
                "INSERT INTO todo (title, is_done) values (?1, ?2)",
                [&add_args.text, &false.to_string()],
            )
            .unwrap();
        }

        ActionType::Done(done_args) => {
            // TODO: Implement the function
        }

        ActionType::UnDone(undone_args) => {
            // TODO: Implement the function
        }

        ActionType::Remove(remove_args) => {
            // TODO: Implement the function
        }
    }
}
