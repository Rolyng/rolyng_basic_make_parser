use clap::{Arg, ArgAction, Command, crate_authors, crate_version, crate_name, crate_description};
use rolyng_basic_make_parser::parse_make;

pub fn main() -> anyhow::Result<()> {
    let matches = Command::new(crate_name!())
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
        .get_matches();
    let myfile = matches.get_one::<String>("file").unwrap();
    let file = std::fs::read_to_string(myfile).expect("Error when reading file");
    let rules = parse_make(&file)?;
    for rule in rules {
        println!("{}", rule);
    }
    Ok(())
}
