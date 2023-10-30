use crossterm::terminal::enable_raw_mode;
use serde::{Deserialize, Serialize};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const DATA_FOLDER: &str = "./data/";
const DATA_DEFAULT_FILE_NAME: &str = "list.json";

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    title: String,
    description: String,
    is_done: bool,
}

#[derive(Clone, Copy, Debug)]
enum MenuItem {
    Home,
    Tasks,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Tasks => 1,
        }
    }
}

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
        }
    });

    Ok(())
}
