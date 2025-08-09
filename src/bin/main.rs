use clap::{Parser, Subcommand};
use lib_dharnadb::{dharanadb::DharanaStore, errors::{SingleResult}};

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

fn main() -> SingleResult<()> {
    let cli = Cli::parse();
    let mut store = DharanaStore::new().unwrap();
    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Set { key, value } => {
            store.set(key.to_owned(), value.to_owned())?;
             println!("✅ Key: {key} ");
        },
        Commands::Get { key } => {
            let get_opt = store.get(key.to_owned())?;

            match get_opt {
                Some(val) =>  println!("📄 {key} : {val} "),
                None => println!("❌ Key: {key} NOT FOUND")
            }
        },
        Commands::Remove { key } => {
         
            store.remove(key.to_owned())?;
               println!("🌋   Key: {key} ");
        }

    }
    Ok(())
}