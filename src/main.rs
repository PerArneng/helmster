
use clap::{Arg, Command};
mod commands;
use commands::init::create_init_command;

fn main() {

    let version = "0.1.0";

    let matches = Command::new("Helmster")
        .version(version)
        .author("Per Arneng <per.arneng@scalebit.com>")
        .about("A tool that generates Helm charts that can run a scripts without the need for a custom base images")
        .subcommands(vec![
            create_init_command(),
        ])
        .get_matches();

    match matches.subcommand_matches("init") {
        Some(matches) => {
            println!("init");
            //println!("name: {:?}", matches.value_of("name").expect("required"));
            if let Some(c) = matches.get_one::<String>("name") {
                println!("Value for -n: {}", c);
            }
        },
        None => {
            println!("no subcommand");
        }
    }


}