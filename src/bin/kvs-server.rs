use clap::Parser;
use ferris::kvstore::KvStore;
use slog::{info, o, Drain, Logger};
use slog_term::PlainSyncDecorator;
use std::{
    env::current_dir,
    io::stdout,
    net::TcpListener,
    thread::scope,
};
use ferris::server::engine::Engine;
use ferris::server::handler::handle_connection;


#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    address: String,

    #[arg(short,long, default_value_t=String::from("Kvs"))]
    engine: String,
}

fn main() {
    // Structured logging with slog
    let plain = PlainSyncDecorator::new(stdout());

    let logger = Logger::root(
        slog_term::FullFormat::new(plain).build().fuse(),
        o!("version" => "0.1"),
    );

    // Parsing arguments from the cli
    let args = Args::parse();
    let engine: Engine = args.engine.into();

    // Opening sled
    let wrapped_db = sled::open("sledlog");

    // Opening KvStore
    let wrapped_store = KvStore::open(current_dir().unwrap().as_path());

    // NOTE: I open both Kvstore and Sled just in case any of them is used
    // They will have seperate logs and data

    // Error Handling, just in case path can't be accessed
    let mut db = match wrapped_db {
        Ok(db) => db,
        Err(e) => panic!("The path cannot be accessed, Error: {}", e),
    };

    let mut store = match wrapped_store {
        Ok(store) => store,
        Err(e) => panic!("The path cannot be accessed, Error: {}", e),
    };

    // Logging intro
    info!(logger,
        "Application started";
        "started_at" => format!("{}", args.address)
    );

    // Binding to the address given
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

    // Match which engine is used
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
