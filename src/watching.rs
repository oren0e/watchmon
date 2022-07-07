use crate::utils::look_for_term_in_file;
use anyhow::Result as AnyhowResult;
use anyhow::{anyhow, Context};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use shlex::Shlex;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

///Monitor a file for the existance of a term, and if the term is not found in the file, trigger
///the command
pub fn monitor_file_for_term(term: &str, filepath: &str, cmd: &str) -> AnyhowResult<()> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(10)).expect("Failed to create watcher");
    watcher
        .watch(&filepath, RecursiveMode::NonRecursive)
        .unwrap_or_else(|_| panic!("Could not watch path {}", &filepath));

    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Write(path) = event {
                    let found = look_for_term_in_file(term, &path)?;
                    if !found {
                        let mut lex = Shlex::new(cmd);
                        let args = lex.by_ref().collect::<Vec<_>>();
                        if lex.had_error {
                            return Err(anyhow!("Failed to parse command"));
                        }
                        let mut comm = Command::new(&args[0]);
                        comm.args(&args[1..]);
                        let mut c = comm
                            .spawn()
                            .with_context(|| format!("Failed running {}", &args[0]))?;
                        c.wait()
                            .with_context(|| format!("Failed running {}", &args[0]))?;
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
