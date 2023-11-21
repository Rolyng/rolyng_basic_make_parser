use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgAction, Command};
use rolyng_basic_make_parser::parse_make;

pub fn main() -> anyhow::Result<()> {
    let mut command = Command::new(crate_name!())
        .author(crate_authors!("\n"))
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .help("Input file")
                .required(false),
        )
        .subcommand(Command::new("author").about("Prints author"));
    let matches = command.clone().get_matches();
    if let Some(_) = matches.subcommand_matches("author") {
        print_author();
        return Ok(());
    }
    if !matches.args_present() {
        let _ = command.print_help();
        return Ok(());
    }
    let myfile = matches.get_one::<String>("file").unwrap();
    let file = std::fs::read_to_string(myfile).expect("Error when reading file");
    let rules = parse_make(&file)?;
    for rule in rules {
        println!("{}", rule);
    }
    Ok(())
}

fn print_author() {
    println!("Author - Denys Honchar");
}
