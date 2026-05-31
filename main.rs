use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("Project Centauri")
        .arg(Arg::new("verbose")
             .short('v')
             .long("verbose")
             .help("Enable verbose mode")
             .action(ArgAction::SetTrue))
        .get_matches();

    if matches.get_flag("verbose") {
        println!("Verbose mode enabled");
    }
}   