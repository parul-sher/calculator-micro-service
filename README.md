# calculator-micro-service
Repository contains code for calculator micro services developed using Python and Rust languages.

## Rust
### Local dev setup
1. Install Rust using [rustup](https://rustup.rs/)
2. Install [Docker](https://docs.docker.com/get-docker/)
3. cd into `rust_calculator` directory
4. Execute `cargo run` to start the server

## Build and Run in local
- cd into `rust_calculator` directory and execute `docker build -t rust_calculator:latest .` to build the docker image
- Execute `docker run -p 8080:8080 rust_calculator:latest` to run the server in local
## Python
