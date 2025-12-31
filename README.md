# Cipher Workshop

A Rust workspace containing implementations of various cipher algorithms, along with a command-line interface (CLI) and a web interface to interact with them.

## Features

- **AES Implementation**: AES-128 in ECB and CBC modes
- **DES Implementation**: DES in ECB mode
- **Command-Line Interface**: Encrypt and decrypt messages or files using the supported ciphers
- **Web Interface**: Browser-based encryption with file upload, drag-and-drop, and random key/IV generation

## Workspace Structure

The `cipher-workshop` workspace is organized into the following crates:

- `aes`: Implementation of the AES cipher (ECB and CBC modes)
- `cipher-core`: Core traits and types for ciphers
- `cipher-factory`: A factory for creating cipher contexts
- `cli`: A command-line interface for the ciphers
- `des`: Implementation of the DES cipher
- `web`: A web interface built with Leptos

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) (for the web interface)
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

The CLI allows you to encrypt and decrypt messages or files using the supported ciphers.

#### AES (ECB mode)

```bash
# Encrypt a message
cargo run --bin cli -- encrypt -a aes -k 0x2B7E151628AED2A6ABF7158809CF4F3C "Hello World"

# Decrypt a message
cargo run --bin cli -- decrypt -a aes -k 0x2B7E151628AED2A6ABF7158809CF4F3C 0x...
```

#### AES-CBC (with IV)

```bash
# Encrypt a file
cargo run --bin cli -- encrypt -a aes-cbc -k 0x2B7E151628AED2A6ABF7158809CF4F3C --iv 0x000102030405060708090A0B0C0D0E0F -i input.txt -o output.enc

# Decrypt a file
cargo run --bin cli -- decrypt -a aes-cbc -k 0x2B7E151628AED2A6ABF7158809CF4F3C --iv 0x000102030405060708090A0B0C0D0E0F -i output.enc -o decrypted.txt
```

#### DES

```bash
# Encrypt a message
cargo run --bin cli -- encrypt -a des -k 0x133457799BBCDFF1 "Hello"

# Decrypt a message
cargo run --bin cli -- decrypt -a des -k 0x133457799BBCDFF1 0x...
```

### Web Interface

The web interface provides a user-friendly way to interact with the ciphers, available at: [https://cryptography.kristofers.xyz/](https://cryptography.kristofers.xyz/)

Features:

- **DES, AES, and AES-CBC** encryption/decryption
- **Random key and IV generation** using Web Crypto API
- **File upload** with drag-and-drop support
- **File download** for encrypted/decrypted output
- **Multiple output formats**: Hex, Binary, Octal, Text

To run the web interface locally:

```bash
cd web
cargo leptos watch
```

## License

This project is licensed under either of the [Apache License, Version 2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT), at your option.
