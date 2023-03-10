use clap::{Arg, Command};

pub fn create_init_command() -> Command {
    Command::new("init")
        .about("Inits a new Helmster project")
        .arg(Arg::new("one")
            .short('o')
            .long("one")
            .required(true)
            .value_name("one")
        )
}