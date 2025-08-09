use clap::{Parser, Subcommand};
use lib_dharnadb::DharanaStore;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set the key for the value
    Set { key: String, value: String },
    /// Get the value
    Get { key : String },
    /// Remove the entry for the key
    Remove {key : String},
}

fn main() {
    let cli = Cli::parse();
    let mut store = DharanaStore::new().unwrap();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Set { key, value } => {
            println!("Setting the {} for {} . . . .", value, key);
            store.set(key.to_owned(), value.to_owned());
        },
        Commands::Get { key } => {
            let getOpt = store.get(key.to_owned());


            match  getOpt {
                Some(val) =>  println!(" {key} : {val} "),
                None => println!("Value not found for {key}")
            }

             
        },
        Commands::Remove { key } => {
            println!("Removing key : {key}");
            store.remove(key.to_owned());
        }

    }
}