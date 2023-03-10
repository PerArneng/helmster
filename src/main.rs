
use clap::{Arg, Command};
mod commands;
use commands::init::init_create;
use crate::commands::init::init_run;

fn main() {

    let version = "0.1.0";

    let matches = Command::new("Helmster")
        .version(version)
        .author("Per Arneng <per.arneng@scalebit.com>")
        .about("A tool that generates Helm charts that can run a scripts without the need for a custom base images")
        .subcommands(vec![
            init_create(),
        ])
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        init_run(matches)
    }

}