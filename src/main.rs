use crate::watching::monitor_file_for_term;
use anyhow::Result as AnyhowResult;
use clap::{crate_version, Arg, Command};

mod utils;
mod watching;

fn main() -> AnyhowResult<()> {
    let matches = Command::new("Watchmon")
        .version(crate_version!())
        .author("Oren Epshtain")
        .about("Utility to watch and monitor for the existence of a specific term in a file and perform a bash command when the existence is false")
        .arg(
            Arg::new("file")
            .short('f')
            .long("file")
            .value_name("FILE PATH")
            .takes_value(true)
            .required(true)
            .help("Path of the file to watch")
            )
        .arg(
            Arg::new("term")
            .short('t')
            .long("text-term")
            .value_name("TEXT TERM")
            .takes_value(true)
            .required(true)
            .help("The text term to look for in the file")
            )
        .arg(
            Arg::new("command")
            .short('c')
            .long("command")
            .value_name("BASH COMMAND")
            .takes_value(true)
            .required(true)
            )
        .arg(
            Arg::new("monitor_fs")
            .short('s')
            .long("fs-monitor")
            .value_name("MONITOR FS")
            .takes_value(false)
            .help("Monitor system files")
            )
        .get_matches();
    let monitor_fs = matches.is_present("monitor_fs");
    monitor_file_for_term(
        matches.value_of("term").expect("`term` is required"),
        matches.value_of("file").expect("`file` is required"),
        matches.value_of("command").expect("`command` is required"),
        monitor_fs,
    )?;
    Ok(())
}
