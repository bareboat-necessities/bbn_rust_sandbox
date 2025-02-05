# NMEA XDR Generator

A Rust library to generate NMEA XDR sentences.

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/bareboat-necessities/bbn_rust_sandbox
   cd bbn_rust_sandbox/rust_xdr
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the example:
   ```bash
   cargo run
   ```

## Example Output

The program generates an NMEA XDR sentence like this:
```
$IIXDR,29.5,P,B,ENGINE,25.0,T,C,COOLING*42
```

## License

This project is licensed under the MIT License.


### Project Structure
Here’s how the project directory should look:

```
nmea-xdr-generator/
├── Cargo.toml
├── Cargo.lock
├── src/
│   └── main.rs
├── README.md
└── .gitignore
```


