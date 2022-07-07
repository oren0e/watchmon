use crate::utils::look_for_term_in_file;
use anyhow::Result as AnyhowResult;
use anyhow::{anyhow, Context};
use notify::event::{DataChange, Event, EventKind, ModifyKind};
use notify::poll::{PollWatcher, PollWatcherConfig};
use notify::{RecursiveMode, Watcher};
use shlex::Shlex;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

///Monitor a file for the existance of a term, and if the term is not found in the file, trigger
///the command
pub fn monitor_file_for_term(
    term: &str,
    filepath: &str,
    cmd: &str,
    monitor_fs_files: bool,
) -> AnyhowResult<()> {
    let (tx, rx) = channel();

    if monitor_fs_files {
        let config = PollWatcherConfig {
            compare_contents: true,
            poll_interval: Duration::from_secs(2),
        };
        let mut watcher = PollWatcher::with_config(tx, config)?;
        watcher.watch(&PathBuf::from(&filepath), RecursiveMode::NonRecursive)?;

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(Event {
                        kind: EventKind::Modify(ModifyKind::Data(DataChange::Any)),
                        ..
                    }) = event
                    {
                        let found = look_for_term_in_file(term, &filepath)?;
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
    } else {
        let mut watcher = notify::recommended_watcher(tx)?;
        watcher.watch(&PathBuf::from(&filepath), RecursiveMode::NonRecursive)?;

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let Ok(Event {
                        kind: EventKind::Modify(ModifyKind::Data(DataChange::Content)),
                        ..
                    }) = event
                    {
                        let found = look_for_term_in_file(term, &filepath)?;
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
}
