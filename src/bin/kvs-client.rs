use bincode::{
    config::{self, Config},
    encode_to_vec,
};
use clap::{Parser, Subcommand};
use serde::Serialize;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

// Cli Parser
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    address: String,
}

#[derive(Subcommand, Serialize)]
enum Commands {
    #[allow(non_camel_case_types)]
    /// Set a key-value pair
    set { key: String, val: String },

    /// Get the value for a key
    #[allow(non_camel_case_types)]
    get { key: String },

    /// Remove a key-value pair
    #[allow(non_camel_case_types)]
    rm { key: String },
}

fn main() {
    let cli = Cli::parse();
    let config = config::standard();

    if cli.command.is_none() {
        Cli::parse_from(["kvs", "--help"]);
        return;
    }

    let mut stream = match TcpStream::connect(cli.address) {
        Ok(stream) => stream,
        Err(e) => {
            panic!("{}", e);
        }
    };

    match cli.command.unwrap() {
        Commands::set { key, val } => {
            let command = [0_u8];

            let bytekey = encode_to_vec(val, config).unwrap();
            let byteval = encode_to_vec(key, config).unwrap();

            println!(
                "{:?} {:?} {:?} {:?} {:?}",
                command.clone(),
                bytekey.len(),
                byteval.len(),
                bytekey.clone(),
                byteval.clone()
            );

            let _ = stream.write(&command);
            let _ = stream.write(&[bytekey.len() as u8]);
            let _ = stream.write(&[byteval.len() as u8]);
            let _ = stream.write(&bytekey[..]);
            let _ = stream.write(&byteval[..]);
        }

        Commands::get { key } => {
            let command = Commands::get {
                key: key.to_string(),
            };
            let bytes = serde_json::to_vec_pretty(&command).unwrap();

            match stream.write_all(&bytes) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }

            // todo!()
        }

        Commands::rm { key } => {
            let command = Commands::rm {
                key: key.to_string(),
            };
            let bytes = serde_json::to_vec_pretty(&command).unwrap();

            match stream.write_all(&bytes) {
                Ok(_) => (),
                Err(e) => panic!("{}", e),
            }
            // todo!()
        }
    }
}
