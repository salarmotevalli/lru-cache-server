# lru-cache-server
# LRU Cache Server with RESP Protocol

## Overview

This project implements a simple in-memory LRU (Least Recently Used) cache server that communicates using the Redis Serialization Protocol (RESP). The server supports basic `SET` and `GET` operations and can be configured via command-line parameters.

## Features

- **LRU Cache Eviction**: Automatically removes least recently used items when capacity is reached
- **RESP Protocol Support**: Compatible with Redis clients for `SET` and `GET` operations
- **Configurable Parameters**:
  - `-p` - Server port (default: 6379)
  - `-c` - Cache capacity in items (default: 100)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/lru-cache-server.git
   cd lru-cache-server
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Starting the Server

```bash
./target/release/lru-cache-server -p 6380 -c 500
```

This starts the server on port 6380 with a cache capacity of 500 items.

### Default Values

If no arguments are provided:
- Port: 6379
- Capacity: 100 items

### Connecting with Redis CLI

```bash
redis-cli -p 6380
```

### Supported Commands

1. **SET** - Store a key-value pair
   ```
   SET mykey "some value"
   ```

2. **GET** - Retrieve a value by key
   ```
   GET mykey
   ```

## Implementation Details

- **LRU Algorithm**: Uses a combination of HashMap and LinkedList for O(1) operations
- **Threading**: Single-threaded async runtime (tokio current_thread)
- **RESP Parser**: Handles simple Redis protocol for basic commands

## Limitations

- Only supports SET and GET operations
- No persistence - cache is lost when server restarts
- Single-threaded (no concurrent access)

## Example Session

```bash
$ redis-cli -p 6380
127.0.0.1:6380> SET test "hello"
OK
127.0.0.1:6380> GET test
"hello"
127.0.0.1:6380> GET nonexistent
(nil)
```
