FROM rust:latest

WORKDIR /usr/src/git-terra-state

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/git-terra-state"]