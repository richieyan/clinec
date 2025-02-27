use clap::{Arg, Command};

pub fn parse_args() -> (Option<String>, Option<String>, bool, bool) {
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
        )
        .arg(
            Arg::new("project_name")
                .value_name("PROJECT_NAME")
                .help("Specifies the project name")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("list_stacks")
                .short('l')
                .long("list-stacks")
                .help("List all supported technology stacks")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let list_stacks = matches.get_flag("list_stacks");

    let tech_stack = matches.get_one::<String>("type").map(|s| s.to_string());
    let project_name = matches.get_one::<String>("project_name").map(|s| s.to_string());
    let verbose = matches.get_flag("verbose");

    (tech_stack, project_name, verbose, list_stacks)
}
