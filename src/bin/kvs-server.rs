use bincode::{config, decode_from_slice, encode_to_vec};
use clap::Parser;
use ferris::{kv_engine::KvEngine, kvstore::KvStore};
use sled::Db;
use slog::{info, o, warn, Drain, Logger};
use slog_term::PlainSyncDecorator;
use std::{
    env::current_dir,
    error::Error,
    fmt::Display,
    io::{stdout, Read, Write},
    net::{TcpListener, TcpStream},
    thread::scope,
    usize,
};

#[derive(Clone, Copy)]
enum Engine {
    Kvs,
    Sled,
}

impl From<Engine> for String {
    fn from(value: Engine) -> Self {
        match value {
            Engine::Kvs => "Kvs".to_string(),
            Engine::Sled => "Sled".to_string(),
        }
    }
}

impl From<String> for Engine {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "Kvs" => Engine::Kvs,
            "Sled" => Engine::Sled,
            _ => panic!("Engine not chosen correctly"),
        }
    }
}

#[derive(Debug)]
enum ServerError {
    UnableToReadFromStream,
    FailedToReadStream { e: Box<dyn Error> },
    UnableToDecodeBytes { e: Box<dyn Error> },
    CommandNotFound,
    GetFoundNone,
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnableToReadFromStream => writeln!(f, "Unable to read from stream"),
            Self::FailedToReadStream { e } => {
                writeln!(f, "Failed to read from stream, Error: {}", e)
            }
            Self::UnableToDecodeBytes { e } => writeln!(f, "UnableToDecodeBytes, Error: {}", e),
            Self::CommandNotFound => writeln!(f, "Command is not found"),
            Self::GetFoundNone => writeln!(f, "Found None"),
        }
    }
}

impl Error for ServerError {}

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
        Err(e) => return Err(ServerError::FailedToReadStream { e: Box::new(e) }),
    }

    let header = Header::new(buf[0], buf[1], buf[2]);
    let mut buf: Vec<u8> = Vec::new();

    match stream.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(e) => return Err(ServerError::FailedToReadStream { e: Box::new(e) }),
    }

    let keybyte = &buf[..{ header.keysize as usize }];

    let valuebyte =
        &buf[{ header.keysize as usize }..{ header.keysize as usize + header.valuesize as usize }];

    let key: String = match decode_from_slice(keybyte, config::standard()) {
        Ok(k) => k.0,
        Err(e) => return Err(ServerError::UnableToDecodeBytes { e: Box::new(e) }),
    };

    let value = decode_from_slice(valuebyte, config::standard());

    let val = match value {
        Ok(val) => Some(val.0),
        Err(_) => None,
    };

    let command = CliCommand::new(header.command, key, val);

    Ok(command)
}

fn execute_command<T: KvEngine>(
    logger: Logger,
    stream: &mut TcpStream,
    store: &mut T,
    parsed: CliCommand,
) -> Result<(), Box<dyn Error>> {
    let command = parsed.command;
    let key = parsed.key;
    let val = parsed.value;
    match command {
        0 => {
            store.tset(key, val.unwrap())?;

            info!(logger, "Application Info"; "Info" => "Set command succesfully ran");
        }
        1 => {
            let res = store.tget(key).unwrap();
            match res {
                Some(l) => {
                    let byte = encode_to_vec(l, config::standard()).unwrap();
                    let _ = stream.write(&[byte.len() as u8]).unwrap();
                    let _ = stream.write(&byte[..]).unwrap();

                    info!(logger, "Application Info"; "Info" => "Get command succesfully ran");
                }
                None => {
                    let byte = encode_to_vec("Cant Get any key from the table", config::standard())
                        .unwrap();
                    let _ = stream.write(&byte[..]);
                    return Err(Box::new(ServerError::GetFoundNone));
                }
            }
        }
        2 => {
            store.tremove(key)?;
            info!(logger, "Application Info"; "Info" => "Remove command succesfully ran");
        }
        _ => {
            return Err(Box::new(ServerError::CommandNotFound));
        }
    }
    Ok(())
}

fn handle_connection<T: KvEngine>(
    stream: &mut TcpStream,
    logger: &Logger,
    store: &mut T,
) {
    let command = handle_listener(stream);
    match command {
        Ok(log) => {
            info!(logger,
                "Incoming Message";
                "Command" =>  format!("{:?}",log)
            );
            let res = execute_command(logger.clone(), stream, store, log);
            match res {
                Ok(_) => (),
                Err(e) => {
                    warn!(logger,
                        "Application Warning";
                        "Error:" => format!("{}",e)
                    );
                }
            }
        }

        Err(e) => {
            warn!(logger,
                "Application Warning";
                "Error:" => format!("{}",e)
            );
        }
    }
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
    let wrapped_db = sled::open("sledlog");

    let logger = Logger::root(
        slog_term::FullFormat::new(plain).build().fuse(),
        o!("version" => "0.1"),
    );

    let args = Args::parse();
    let engine: Engine = args.engine.into();

    let mut store = KvStore::open(current_dir().unwrap().as_path()).unwrap();

    let mut db = match wrapped_db {
        Ok(db) => db,
        Err(e) => panic!("The path cannot be accessed, Error: {}", e),
    };

    info!(logger,
        "Application started";
        "started_at" => format!("{}", args.address)
    );

    let listener = match TcpListener::bind(args.address) {
        Ok(l) => l,
        Err(e) => {
            info!(logger,
                "Application Warning";
                "Error:"  => format!("{}",e)
            );
            panic!()
        }
    };

    match engine {
        Engine::Kvs => {
            for stream_wrapped in listener.incoming() {
                let mut stream = stream_wrapped.unwrap();
                scope(|scope| {
                    scope.spawn(|| handle_connection(&mut stream, &logger, &mut store));
            });
    }
        },
        Engine::Sled => {
            for stream_wrapped in listener.incoming() {
                let mut stream = stream_wrapped.unwrap();
                scope(|scope| {
                    scope.spawn(|| handle_connection(&mut stream, &logger, &mut db));
                });
            }
        }
    }

}
