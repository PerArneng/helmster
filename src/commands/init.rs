use clap::{Arg, Command};

pub fn init_create() -> Command {
    Command::new("init")
        .about("Inits a new Helmster project")
        .arg(Arg::new("name")
            .short('n')
            .long("name")
            .required(true)
            .value_name("name")
        )
}

pub fn init_run(matches: &clap::ArgMatches) {
    println!("init");
    //println!("name: {:?}", matches.value_of("name").expect("required"));
    if let Some(c) = matches.get_one::<String>("name") {
        println!("Value for -n: {}", c);
    }
}