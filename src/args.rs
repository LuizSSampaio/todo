use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub action_type: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Add an item to the list
    Add(AddCommand),

    /// Remove an item from the list
    Remove(RemoveCommand),

    /// Mark an item as done
    Done(DoneCommand),

    /// Mark an item as to be done
    UnDone(UnDoneCommand),
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The text of the item(Can be splited using ","[COMMA])
    pub text: String,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// The ID of the item(Can be splited using ","[COMMA])
    pub id: String,
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    /// The ID of the item(Can be splited using ","[COMMA])
    pub id: String,
}

#[derive(Debug, Args)]
pub struct UnDoneCommand {
    /// The ID of the item(Can be splited using ","[COMMA])
    pub id: String,
}
