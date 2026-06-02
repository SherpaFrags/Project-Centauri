use clap::{Arg, ArgAction, Command};


fn main() {
    let mut verbose = false;
    println!("\nProject Centauri CLI\n______________________________\n");

    let matches = Command::new("Project Centauri")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose mode")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Specify the output file")
                .value_name("FILE")
                .action(ArgAction::Set)
                .required(false),
        )
        .arg(
            Arg::new("kernel")
                .short('k')
                .long("kernel-version")
                .help("Specify the kernel version to compile.")
                .value_name("KERNEL-VERSION")
                .action(ArgAction::Set)
                .required(true),
        )
        .get_matches();

    if matches.get_flag("verbose") {
        println!("Verbose mode enabled");
        verbose = true;
    }

    if let Some(output_file) = matches.get_one::<String>("output") {
        if verbose {
            println!("Output file specified: {}", output_file);
        }
    }

    if let Some(kernel_version) = matches.get_one::<String>("kernel") {
        if verbose {
            println!("Kernel version specified: {}", kernel_version);
        }
    }
}
