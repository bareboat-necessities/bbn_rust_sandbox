# NMEA XDR Generator

A Rust library to generate NMEA XDR sentences.

## Usage

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/nmea-xdr-generator.git
   cd nmea-xdr-generator
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

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
```

---

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

---

### `.gitignore`
Add a `.gitignore` file to exclude unnecessary files from version control.

```gitignore
# Ignore Cargo build artifacts
/target/
**/*.rs.bk

# Ignore Cargo lock file
Cargo.lock

# Ignore IDE-specific files
.vscode/
.idea/
