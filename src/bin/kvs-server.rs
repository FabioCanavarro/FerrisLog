use clap::Parser;
use ferris::concurrency::ThreadPool;
use ferris::{concurrency::NaiveThreadPool, kvstore::KvStore};
use ferris::server::engine::Engine;
use ferris::server::handler::handle_connection;
use sled::Db;
use slog::{info, o, Drain, Logger};
use slog_term::PlainSyncDecorator;
use std::sync::{Arc, Mutex};
use std::{env::current_dir, io::stdout, net::TcpListener, thread::{self, scope}};
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    addr: String,

    #[arg(short,long, default_value_t=String::from("Kvs"))]
    engine: String,
}

fn main() {
    // Parsing arguments from the cli
    let args = Args::parse();

    lazy_static!(
        static ref LOGGER: Logger  = {
            let plain = PlainSyncDecorator::new(stdout());
            Logger::root(
                slog_term::FullFormat::new(plain)
                                                .build()
                                                .fuse(),
                o!(
                    "version" => "0.1",
                )
            )
        };
        pub static ref DB: Arc<Mutex<Db>> = {
            let wrapped_db = sled::open("sledlog");
            let db = match wrapped_db {
                Ok(db) => db,
                Err(e) => panic!("The path cannot be accessed, Error: {}", e),
            };
            Arc::new(Mutex::new(db))
        };

        pub static ref STORE: Arc<Mutex<KvStore>> = {
            let wrapped_store = KvStore::open(current_dir().unwrap().as_path());
            let store = match wrapped_store {
                Ok(store) => store,
                Err(e) => panic!("The path cannot be accessed, Error: {}", e),
            };
            Arc::new(Mutex::new(store))
        };
    );

    info!(LOGGER,
        "Application started";
        "started_at" => format!("{}", args.addr),
        "Engine" => &args.engine
    );


    let engine: Engine = args.engine.into();

    // Opening sled

    // Opening KvStore

    // NOTE: I open both Kvstore and Sled just in case any of them is used
    // They will have seperate logs and data

    // Error Handling, just in case path can't be accessed

    // Binding to the address given
    let listener = match TcpListener::bind(args.addr) {
        Ok(l) => l,
        Err(e) => {
            info!(LOGGER,
                "Application Warning";
                "Error:"  => format!("{}",e)
            );
            panic!()
        }
    };

    let naive_pool = NaiveThreadPool::new(4).expect("Failed to create NaiveThreadPool");

    // Match which engine is used
    match engine {
        Engine::Kvs => {
            let kvstore_thread = Arc::clone(&STORE);
            for stream_wrapped in listener.incoming() {
                let mut stream = stream_wrapped.unwrap();
                let mut store_guard = kvstore_thread.lock().unwrap();
                naive_pool.spawn(move || {handle_connection(&mut stream, &LOGGER, &mut *store_guard)})
            }
        }
        Engine::Sled => {
            let sled_thred = Arc::clone(&DB);
            for stream_wrapped in listener.incoming() {
                let mut stream = stream_wrapped.unwrap();
                let mut sled_guard = sled_thred.lock().unwrap();
                scope(|scope| {
                    scope.spawn(move || handle_connection(&mut stream, &LOGGER, &mut *sled_guard));
                });
            }
        }
    }
}
