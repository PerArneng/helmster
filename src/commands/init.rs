use clap::{Arg, Command};

pub fn create_init_command() -> Command {
    Command::new("init")
        .about("Inits a new Helmster project")
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .required(true)
            .value_name("name")
        )
}