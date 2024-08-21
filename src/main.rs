use std::path::PathBuf;

use clap::{ArgEnum, IntoApp, Parser, Subcommand, ValueHint};
use clap_complete::{generate, Generator, Shell};

/// Greet command (example for clap_complete command).
#[derive(Parser,Debug)]
struct Cli {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand,Debug)]
enum Action {
    /// Greet some message.
    Greet {
        /// Language in which messages are shown.
        #[clap(long,short,arg_enum)]
        language: Option<Language>,
        /// File whose content is printed.
        ///
        /// The trailing whitespaces of the content are trimmed.
        #[clap(long,short,conflicts_with("language"),value_hint(ValueHint::FilePath))]
        file: Option<PathBuf>,
    },
    /// Generate completion script
    Completion {
        #[clap(long,short,arg_enum)]
        shell: Shell,
    },
}

#[derive(ArgEnum,Clone,Debug)]
enum Language {
    En,
    Ja,
}

impl Action {
    fn handle(self) {
        use Action::*;

        match self {
            Greet { language: None, file: None } => {
                println!("Hello");
            },
            Greet { language: Some(Language::En), .. } => {
                println!("Hello");
            },
            Greet { language: Some(Language::Ja), .. } => {
                println!("こんにちは");
            },
            Greet { file: Some(file), .. } => {
                let s = std::fs::read_to_string(&file).unwrap();
                println!("{}", s.trim_end());
            },
            Completion { shell } => {
                print_completer(shell);
            },
        }
    }
}

fn print_completer<G: Generator>(generator: G) {
    let mut app = Cli::into_app();
    let name = app.get_name().to_owned();

    generate(generator, &mut app, name, &mut std::io::stdout());
}

fn main() {
    Cli::parse().action.handle();
}