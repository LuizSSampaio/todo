use args::{ActionType, TodoArgs};
use clap::Parser;

mod args;

fn main() {
    let args = TodoArgs::parse();

    println!("{:?}", args);
}
