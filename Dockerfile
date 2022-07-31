FROM rust

WORKDIR /bin/rust-api

COPY ./ /bin/rust-api

RUN cargo build --release

CMD ["./target/release/rust-api"]
