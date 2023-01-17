# Whoopty CLI
This is a command line tool written in Rust that performs whois lookups.

## Installation
To install Whois CLI, you will need Rust installed on your machine.

1. Clone this repository: `git clone https://github.com/harrisrobin/whoopty.git`
2. Change into the directory: `cd whoopty`
3. Build the binary: `cargo build --release`
4. The binary will be located in `target/release/whoopty`

## Usage
The usage of Whoopty CLI is very simple. Just pass the domain you want to lookup as an argument when running the binary.

```bash
whoopty -- -d example.com
```

## Output
The output of Whois CLI is the whois data of the domain. It will be printed to standard output.

```
Registrar: Example Registrar
Registrant Name: John Doe
Registrant Email: john@example.com
```