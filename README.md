# BuzzNetScan: The Bee Advanced Network Scanner
BuzzNetScan is a simple CLI tool for scanning IP ranges to find active hosts. It features a colorful banner and allows users to specify custom IP ranges for scanning.                                                                                                                                                                              
                                                                                                                                                                              
## Requirements                                                                                                                                                                              
                                                                                                                                                                              
- Rust Programming Language                                                                                                                                                                              
- Cargo (Rust's package manager and build system)                                                                                                                                                                              
                                                                                                                                                                              
## Installation                                                                                                                                                                              
                                                                                                                                                                              
First, ensure you have Rust and Cargo installed on your system. You can download them from [https://rustup.rs/](https://rustup.rs/).                                                                                                                                                                              
                                                                                                                                                                              
Clone the BuzzNetScan repository:                                                                                                                                                                              
                                                                                                                                                                              
```bash                                                                                                                                                                              
git clone https://github.com/yourusername/buzznetscan.git                                                                                                                                                                              
cd buzznetscan
```

Build the project using Cargo:

```bash
cargo build --release
```
                                                                                                                                                                            
The executable will be located in ./target/release/.

## Usage
To scan an IP range, run:

```bash
./target/release/buzznetscan <start_ip> <end_ip>
```                                                                                                                                                                        
For example, to scan the range 10.0.0.1 to 10.0.0.255, use:

```bash
./target/release/buzznetscan 10.0.0.1 10.0.0.255
```
                                                                                                                                                                            
## Author
Abeis Nelson

## License
MIT License
