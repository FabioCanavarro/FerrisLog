use bincode::{config, decode_from_slice};
use clap::Parser;
use ferris::kvstore::{error::KvError, KvStore};
use slog::{info, o, warn, Drain, Logger};
use slog_term::PlainSyncDecorator;
use std::{
    env::current_dir, error::Error, fmt::Display, io::{stdout, Read, Write}, net::{TcpListener, TcpStream}, usize
};


#[derive(Clone, Copy)]
enum Engine {
    Kvs,
    Sled,
}

#[derive(Debug)]
enum ServerError {
    UnableToReadFromStream,
    FailedToReadStream {e:Box<dyn Error>},
    UnableToDecodeBytes {e:Box<dyn Error>},
    CommandNotFound,

}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnableToReadFromStream => writeln!(f, "Unable to read from stream"),
            Self::FailedToReadStream { e } => writeln!(f,"Failed to read from stream, Error: {}", e),
            Self::UnableToDecodeBytes { e } => writeln!(f,"UnableToDecodeBytes, Error: {}", e),
            ServerError::CommandNotFound => writeln!(f,"Command is not found"),
        }
    }
}

impl Error for ServerError {}

impl From<Engine> for String {
    fn from(value: Engine) -> Self {
        match value {
            Engine::Kvs => "Kvs".to_string(),
            Engine::Sled => "Sled".to_string(),
        }
    }
}

struct Header {
    command: u8,
    keysize: u8,
    valuesize: u8,
}

#[derive(Debug)]
struct CliCommand {
    command: u8,
    key: String,
    value: Option<String>,
}

impl CliCommand {
    fn new(command: u8, key: String, value: Option<String>) -> CliCommand {
        CliCommand {
            command,
            key,
            value,
        }
    }
}

impl Header {
    fn new(command: u8, keysize: u8, valuesize: u8) -> Header {
        Header {
            command,
            keysize,
            valuesize,
        }
    }
}

fn handle_listener(stream: &mut TcpStream) -> Result<CliCommand, ServerError> {
    let mut buf: [u8; 3] = [0, 0, 0];

    let _ = stream.flush();

    match stream.read_exact(&mut buf) {
        Ok(_) => (),
        Err(e) => return Err(ServerError::FailedToReadStream { e: Box::new(e) })
    }

    let header = Header::new(buf[0], buf[1], buf[2]);
    let mut buf: Vec<u8> = Vec::new();

    match stream.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(e) => return Err(ServerError::FailedToReadStream { e: Box::new(e) })
    }

    let keybyte = &buf[..{ header.keysize as usize }];

    let valuebyte = &buf[{ header.keysize as usize }..{ header.keysize as usize + header.valuesize as usize }];

    let key: String = 
        match decode_from_slice(keybyte, config::standard()) {
            Ok(k) => k.0,
            Err(e) => return Err(ServerError::UnableToDecodeBytes { e: Box::new(e) })
        };

    let value = decode_from_slice(valuebyte, config::standard());

    let val = match value {
        Ok(val) => Some(val.0),
        Err(_) => None,
    };

    let command = CliCommand::new(header.command, key, val);

    Ok(command)
}

fn execute_command(kvstore: &mut KvStore, parsed: CliCommand) -> Result<(), Box<dyn Error>>{
    let command = parsed.command;
    let key = parsed.key;
    let val = parsed.value;
    match command {
        0 => {
            let res = kvstore.set(key, val.unwrap());
            if let Err(e) = res {return Err(Box::new(e));}
        },
        1 => {
            let res = kvstore.get(key);
            if let Err(e) = res {return Err(Box::new(e));}
        },
        2 => {
            let res = kvstore.remove(key);
            if let Err(e) = res {return Err(Box::new(e));}
        },
        _ => {
            return Err(Box::new(ServerError::CommandNotFound));
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    address: String,

    #[arg(short,long, default_value_t=String::from("Kvs"))]
    engine: String,
}

fn main() {
    // Structured Logging
    let plain = PlainSyncDecorator::new(stdout());

    let logger = Logger::root(
        slog_term::FullFormat::new(plain).build().fuse(),
        o!("version" => "0.1"),
    );

    let args = Args::parse();
    let mut store = KvStore::open(current_dir().unwrap().as_path()).unwrap();

    // Initial logging
    info!(logger,
        "Application started";
        "started_at" => format!("{}", args.address)
    );

    let listener =
        match TcpListener::bind(args.address) {
            Ok(l) => l, 
            Err(e) => {
                    info!(logger,
                        "Application Warning";
                        "Error:"  => format!("{}",e)
                    );
                    panic!()
            }
        };

    for stream in listener.incoming() {
        let command = handle_listener(&mut stream.unwrap());
        match command {
            Ok(log) => {
                    info!(logger,
                        "Incoming Message";
                        "Command" =>  format!("{:?}",log)
                    );
                    let res = execute_command(&mut store, log);
                    match res{
                        Ok(_) => (),
                        Err(e) => {
                            warn!(logger,
                                "Application Warning";
                                "Error:" => format!("{}",e)
                            );
                        }
                    }
                },

            Err(e) => {
                    warn!(logger,
                        "Application Warning";
                        "Error:" => format!("{}",e)
                    );
                }
        }
    
    }
}
