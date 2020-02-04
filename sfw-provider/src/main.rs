use clap::{App, ArgMatches, SubCommand};

pub mod built_info;
mod commands;
mod config;
pub mod provider;

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    println!("{}", banner());

    let arg_matches = App::new("Nym Service Provider")
        .version(built_info::PKG_VERSION)
        .author("Nymtech")
        .about("Implementation of the Loopix-based Service Provider")
        .subcommand(commands::run::command_args())
        .get_matches();

    execute(arg_matches);
}

fn execute(matches: ArgMatches) {
    match matches.subcommand() {
        ("run", Some(m)) => commands::run::execute(m),
        _ => println!("{}", usage()),
    }
}

fn usage() -> String {
    banner() + "usage: --help to see available options.\n\n"
}

fn banner() -> String {
    format!(
        r#"

      _ __  _   _ _ __ ___
     | '_ \| | | | '_ \ _ \
     | | | | |_| | | | | | |
     |_| |_|\__, |_| |_| |_|
            |___/

             (store-and-forward provider - version {:})

    "#,
        built_info::PKG_VERSION
    )
}
