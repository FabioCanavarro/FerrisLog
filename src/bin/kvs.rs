extern crate clap;
use std::{fs::File, path::PathBuf, process::exit};

use clap::{Parser, Subcommand};
use kvs::kvstore::KvStore;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    set {
        key: String,
        val: String,
    },
    get {
        key: String,
    },
    rm {
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut store = match KvStore::open(&PathBuf::from("log.txt")) {
        Ok(_) => KvStore::open(&PathBuf::from("log.txt")).unwrap(),
        Err(_) => {
            let _ = File::create("log.txt");
            KvStore::open(&PathBuf::from("log.txt")).expect("Error is here")
        }
    };
    // Your implementation here
    match &cli.command.unwrap() {
        Commands::get { key } => {
            let val = store.get(key.to_string());
            match val.unwrap(){
                Some(d) => println!("{}",d),
                None => println!("Key not found")
            }
        },
        Commands::rm { key } => {
            let res = store.remove(key.to_string());
            match res{
                Ok(_) => (),
                Err(_) => {println!("Key not found");exit(1);},
            }
        },
        Commands::set { key, val } => {
            let res = store.set(key.to_string(), val.to_string());
            
        }
    }
}
