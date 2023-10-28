use args::{ActionType, TodoArgs};
use clap::Parser;

mod args;
mod db;

fn main() {
    let args = TodoArgs::parse();

    let db_connection = db::connect();

    match args.action_type {
        ActionType::New(_new_args) => {
            db::create_new_list(db_connection);
        }

        ActionType::Add(add_args) => {
            // TODO: Implement the function
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
