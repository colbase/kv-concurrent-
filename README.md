# kv-concurrent 🚀

A small, educational concurrent key-value TCP server in Rust.

This project implements a simple in-memory key-value store that accepts plain-text commands over TCP. It's designed to demonstrate basic concurrency (thread-per-connection) using an `Arc<Mutex<HashMap<_, _>>>` and a minimal custom protocol for GET/SET/DEL operations.

## Features ✅

- Simple, human-readable line-based protocol
- Concurrent clients handled with threads
- In-memory storage using `HashMap` protected by `Mutex`
- Minimal dependency surface (std only)

## Protocol & Commands 🔧

All commands are single-line and terminated by a newline (`\n`). Arguments are separated by whitespace.

- `GET <key>` — returns the value or `not found` if missing
- `SET <key> <value>` — sets a key to a value and responds with `OK`
- `DEL <key>` — deletes a key and responds with the deleted value or `not found`

Note: Commands are case-sensitive (expect `GET`, `SET`, `DEL`).

## Build & Run ⚡

Requirements: Rust (stable) toolchain.

Build and run locally:

```bash
# from the project root
cargo run
```

By default the server listens on `127.0.0.1:7978` and prints a message on startup:

```
concurrent server listening on port localhost:7978
```

## Example Usage (with netcat) 🧪

Open a new terminal and run:

```bash
# set a key
printf "SET mykey hello\n" | nc 127.0.0.1 7978
# => OK

# get the key
printf "GET mykey\n" | nc 127.0.0.1 7978
# => hello

# delete the key
printf "DEL mykey\n" | nc 127.0.0.1 7978
# => deleted hello
```

You can also connect with `telnet` or any TCP client and issue newline-terminated commands interactively.

## Limitations & Notes ⚠️

- This is an educational example, not intended for production use.
- The server uses a global `Mutex`, so concurrent access is serialized.
- No persistence — data is lost when the process exits.
- Error messages and command validation are intentionally minimal.

## Contributing 🤝

Contributions and improvements are welcome. Open an issue or submit a pull request with changes or ideas.

## License 📄

MIT — see `LICENSE` (or add one) for details.

---

Thanks for checking out `kv-concurrent`! If you'd like, I can also:

- add example client scripts, or
- add a small test harness that exercises commands concurrently.
