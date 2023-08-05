# calculator-micro-service
Repository contains code for calculator micro services developed using Python and Rust languages.

## Rust
### Local dev setup
1. Install Rust using [rustup](https://rustup.rs/)
2. Install [Docker](https://docs.docker.com/get-docker/)
3. cd into `rust_calculator` directory
4. Execute `cargo run` to start the server

### Build and Run in local
- cd into `rust_calculator` directory and execute `docker build -t rust_calculator:latest .` to build the docker image
- Execute `docker run -p 8080:8080 rust_calculator:latest` to run the server in local

### Docker image size evolution:
- 5 August 2023 - After adding `add()` function and `serde` dependency.
```commandline
REPOSITORY       TAG      IMAGE ID       SIZE
rust_calculator  latest   2aac2868b938   28.9MB
```
- 5 August 2023 - Initial image size with distroless image. This image contains only health check service implemented.
```commandline
REPOSITORY       TAG     IMAGE ID       SIZE
rust_calculator  latest  efcf1758c197   28.8MB
```

## Python
