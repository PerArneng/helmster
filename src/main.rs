
use clap::{Arg, Command};
mod commands;
use commands::init::create_init_command;

fn main() {

    let version = "0.1.0";

    let matches = Command::new("Helmster")
        .version(version)
        .author("Per Arneng <per.arneng@scalebit.com>")
        .about("A tool that generates Helm charts that can run a scripts without the need for a custom base images")
        .arg(Arg::new("one")
            .short('o')
            .long("one")
            .required(true)
            .value_name("one"))
        .subcommands(vec![
            create_init_command(),
        ])
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );

}