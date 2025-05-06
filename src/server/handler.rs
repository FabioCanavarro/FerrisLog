use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
};

use bincode::{config, decode_from_slice, encode_to_vec};
use slog::{info, warn, Logger};

use crate::kv_engine::KvEngine;

use super::error::ServerError;

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
    /*
     * Reads data from the TcpStream and parse them into the CliCommand struct
     */
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
    /*
     * Executes the command based on the parsed CliCommand,
     * Logs to the command executed, their outputs and their inputs to the logger
     */
    let command = parsed.command;
    let key = parsed.key;
    let val = parsed.value;
    match command {
        0 => {
            store.tset(key, val.unwrap())?;

            info!(logger, "Application Info"; "Info" => "Set command succesfully ran");
        }
        1 => {
            let res = store.tget(key);
            match res {
                Ok(l) => {
                    let l = l.unwrap();
                    let byte = encode_to_vec(l, config::standard())
                        .unwrap_or("Get Error Found None".as_bytes().to_vec());
                    let _ = stream.write(&[byte.len() as u8]).unwrap();
                    let _ = stream.write(&byte[..]).unwrap();

                    info!(logger, "Application Info"; "Info" => "Get command succesfully ran");
                    info!(logger, "Application Info"; "Info" => format!("Sent back {:?}",&byte));
                }
                Err(e) => {
                    let byte = encode_to_vec("Cant Get any key from the table", config::standard())
                        .unwrap();
                    let _ = stream.write(&byte[..]);
                    warn!(logger,
                        "Application Warning";
                        "Error:" => format!("{:?} and {:?}",ServerError::GetFoundNone,e)
                    );
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

pub fn handle_connection<T: KvEngine>(stream: &mut TcpStream, logger: &Logger, store: &mut T) {
    /*
     * The base function that handles the connection
     */
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
