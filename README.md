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
- curl `http://localhost:8080/health` to check the health of the server
- curl `http://localhost:8080/add?first=1&second=2` to add two numbers
- curl `http://localhost:8080/subtract?first=1&second=2` to subtract two numbers
- curl `http://localhost:8080/multiply?first=1&second=2` to multiply two numbers
- curl `http://localhost:8080/divide?first=1&second=2` to divide two numbers

### Docker image size evolution:
- 5 August 2023 - After adding `add()`, `subtract()`, `multiply()`, `divide()` functions and `serde` dependency.
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
### Local dev setup
- We will use conda to create a virtual environment and install dependencies.
- Install conda using [conda](https://docs.conda.io/projects/conda/en/latest/user-guide/install/)
- cd into `python_calculator` directory
- Execute below commands to setup the environment
```commandline
conda create -n python_calculator python=3.10
conda activate python_calculator
pip install -r requirements.txt -r requirements-test.txt
```
### Build and Run in local
- cd into `python_calculator` directory
- Execute `flask --app src/main.py run` to start the server

### Build and run in Docker:
- cd into `python_calculator` directory
- Execute `docker build -t python_calculator:latest .` to build the docker image
- Execute `docker run -p 5000:5000 python_calculator:latest` to run the server in docker container

### Run in local or in docker:
- curl `http://localhost:5000/health` to check the health of the server
- curl `http://localhost:5000/add?num1=1&num2=2` to add two numbers
- curl `http://localhost:5000/subtract?num1=1&num2=2` to subtract two numbers
- curl `http://localhost:5000/multiply?num1=1&num2=2` to multiply two numbers
- curl `http://localhost:5000/divide?num1=1&num2=2` to divide two numbers

### Docker image size evolution-vasu:
- 5 August 2023 - After adding `add()`, `subtract()`, `multiply()`, `divide()` functions and `flask` dependency.
```commandline
REPOSITORY          TAG       IMAGE ID       SIZE
python_calculator   latest    5e9998a179e4   1.02GB
```
