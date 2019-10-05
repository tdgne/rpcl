use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use clap;

mod repository;
use repository::{RepositoryStore};

mod collector;
use collector::*;

mod tui;
use tui::*;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("Repository Locator")
        .version("0.1.0")
        .author("tdgne")
        .about("Locates repositories")
        .arg(clap::Arg::with_name("DIR")
             .help("Sets the root directory to start searching")
             .index(1))
        .get_matches();
    let root_path = matches.value_of("DIR").unwrap_or(".").to_owned();
    let (tx, rx) = channel();
    let repositories = RepositoryStore::new();
    {
        let repositories = repositories.clone();
        let root_path = root_path.clone();
        let _collector = thread::spawn(|| {
            collect_repositories(root_path, repositories, tx).unwrap();
        });
    }

    let (spinner_tx, spinner_rx) = channel();
    let _spinner = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(333));
            spinner_tx.send(()).unwrap();
        }
    });

    run_tui(repositories, root_path, rx, spinner_rx)?;
    Ok(())
}


