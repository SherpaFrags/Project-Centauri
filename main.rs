use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("My Program")
        .arg(Arg::new("verbose")
             .short('v')
             .long("verbose")
             .help("Enable verbose mode")
             .action(ArgAction::SetTrue)) // Replaces .takes_value(false)
        .get_matches();

    if matches.get_flag("verbose") { // Replaces .is_present("verbose")
        println!("Verbose mode enabled");
    }
}   