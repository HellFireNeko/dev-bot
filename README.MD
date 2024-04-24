# Dev bot
A discord bot that using the assitance of [Docker](https://www.docker.com/) should be able to run without trouble!

## Requirements
For Windows or OSX, you will need the following software:
- [Docker Desktop](https://www.docker.com/products/docker-desktop/) Any version should work!

For Linux you will only need to install [Rust](https://rustup.rs/).

Linux users can optionally use Docker as well.

## How to build
For Windows and OSX (And linux users wanting to use the docker):
1. Download and install [Docker Desktop](https://www.docker.com/products/docker-desktop/)
2. Clone the repo `git clone https://github.com/HellFireNeko/dev-bot`
3. Add your bot token in `\secrets\token.txt` in the root of the project
4. Build the docker container `docker build -t dev-bot .`
5. Run the docker container `docker run -p 80:80 dev-bot`

For Linux:
1. Clone the repo `git clone https://github.com/HellFireNeko/dev-bot`
2. Add your bot token in `\secrets\token.txt` in the root of the project
3. Navigate to the project
4. Compile and run the bot:
    - For debug mode: `cargo run`
    - For release mode: `cargo run --release`