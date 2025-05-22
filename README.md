# Ferrislog-ServerClient

[![Rust](https://img.shields.io/badge/Rust-1.72%2B-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

[![Rust](https://github.com/FabioCanavarro/FerrisLog-ServerClient/actions/workflows/-rust.yml/badge.svg)](https://github.com/FabioCanavarro/FerrisLog-ServerClient/actions/workflows/-rust.yml)

A persistent, log-structured key-value store implemented in Rust with a friendly CLI interface. Designed for reliability and simplicity.

## Features

- **Network Client Support**: Operations can be called from a device to a server
- **Core Operations**: Set, get, and remove key-value pairs with easy commands
- **Persistence**: All operations are logged as JSON to survive program restarts
- **Automatic Log Compaction**: Automatic compaction when log size exceeds threshold
- **Snapshots**: Create and load snapshots for backup and recovery
- **Command Line Interface**: Built with `clap` for intuitive command parsing
- **Automatic Separation**: If the server address isn't given, the log will be saved in the local device

## Installation

```bash
# Clone the repository
git clone https://github.com/FabioCanavarro/Ferrislog
cd Ferrislog

# Build with Cargo
cargo build --release

# Install globally
cargo install --path .
```

## Usage

### Server

```bash
# Setup the server in 127.0.0.1:8080
kvs-server --addr 127.0.0.1:8080

# Setup the server in 127.0.0.1:8080 with the KvEngine
kvs-server --addr 127.0.0.1:8080 --engine Kvs

# Setup the server in 127.0.0.1:8080 with Sled
kvs-server --addr 127.0.0.1:8080 --engine sled
```


### Client

```bash
# Set a key-value pair
kvs-client --addr 127.0.0.1:8080 set username ferris
# Output: Key set successfully

# Get the value for a key
kvs-client --addr 127.0.0.1:8080 get username
# Output: ferris

# Remove a key
kvs-client --addr 127.0.0.1:8080 rm username
# Output: Key removed successfully

# Try to get a non-existent key
kvs-client --addr 127.0.0.1:8080 get username
# Output: Key not found
```

## Implementation Details

### Storage Architecture
Ferrislog uses a log-structured storage model:

1. All operations (set, remove) are appended to a log file

2. An in-memory hash map tracks positions of the latest value for each key

3. On startup, the store rebuilds its state by replaying the log

4. Periodic compaction removes redundant entries to keep the log size manageable

## Performance Considerations

- Log Compaction: Automatically triggers when log exceeds 1024 bytes

- Memory Usage: Keeps only key pointers in memory, not values

- Recovery: Rebuilds state on startup by replaying the log

## Future Enhancements

- Multi-threaded operations for better performance

- Time-to-live (TTL) for keys

- Set the compaction value in kvs-address

- Encryption when sending data


