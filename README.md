# Cipher Workshop

This is a Rust workspace containing implementations of various cipher algorithms, along with a command-line interface (CLI) and a web interface to interact with them.

## Features

- **AES and DES Implementations**: The workspace includes implementations of the Advanced Encryption Standard (AES) and the Data Encryption Standard (DES).
- **Command-Line Interface**: A CLI to encrypt and decrypt messages using the supported ciphers.
- **Web Interface**: A web-based interface to perform cipher operations in the browser.

## Workspace Structure

The `cipher-workshop` workspace is organized into the following crates:

- `aes`: Implementation of the AES cipher.
- `cipher-core`: Core traits and types for ciphers.
- `cipher-factory`: A factory for creating cipher contexts.
- `cli`: A command-line interface for the ciphers.
- `des`: Implementation of the DES cipher.
- `web`: A web interface for the ciphers, built with Leptos.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install) (for the web interface)
- [Node.js](https://nodejs.org/en/download/) (for end-to-end testing)

### Building the Project

Clone the repository and build the workspace:

```bash
git clone https://github.com/kristofers-solo/cipher-workshop.git
cd cipher-workshop
cargo build
```

## Usage

### CLI

The CLI allows you to encrypt and decrypt messages using the supported ciphers.

#### AES

To encrypt a message with AES:

```bash
cargo run --bin cli -- encrypt -a aes -k <KEY> <MESSAGE>
```

To decrypt a message with AES:

```bash
cargo run --bin cli -- decrypt -a aes -k <KEY> <MESSAGE> 
```

#### DES

To encrypt a message with DES:

```bash
cargo run --bin cli -- encrypt -a des -k <KEY> <MESSAGE>
```

To decrypt a message with DES:

```bash
cargo run --bin cli -- decrypt -a des -k <KEY> <MESSAGE>
```

### Web Interface

The web interface provides a user-friendly way to interact with the ciphers, available at: [https://cryptography.kristofers.xyz/](https://cryptography.kristofers.xyz/)

To run the web interface, navigate to the `web` directory and use `trunk`:

```bash
cd web
trunk serve --open
```

## License

This project is licensed under either of the [Apache License, Version 2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT), at your option.
