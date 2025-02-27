use clap::{Arg, Command};

pub fn parse_args() -> (String, String, bool) {
    let matches = Command::new("clinec")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Generates AI configuration files for Cline programming plugins")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TECH_STACK")
                .help("Specifies the technology stack")
                .required(true)
        )
        .arg(
            Arg::new("project_name")
                .value_name("PROJECT_NAME")
                .help("Specifies the project name")
                .required(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let tech_stack = matches.get_one::<String>("type").unwrap().to_string();
    let project_name = matches.get_one::<String>("project_name").unwrap().to_string();
    let verbose = matches.get_flag("verbose");

    (tech_stack, project_name, verbose)
}
