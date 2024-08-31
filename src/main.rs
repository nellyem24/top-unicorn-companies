mod top_countries;

fn main() {
    let cmd = clap::Command::new("uni")
        .bin_name("uni")
        .styles(CLAP_STYLING)
        .subcommand_required(true)
        .subcommand(
            clap::command!("example").arg(
                clap::arg!(--"manifest-path" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf)),
            ),
        )
        .subcommand(clap::command!("bycountry"));

    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("example", matches)) => matches,
        Some(("bycountry", _)) => {
            top_countries::get_top_ten_unicorn_countries();
            return;
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
    println!("{manifest_path:?}");
}

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

fn bycountry() {
    // TODO: read csv 
    
    // get the top 10 countries with more startups
    println!("bycountry");
}