FROM rust:1-bullseye as builder

WORKDIR /app

COPY ${pwd} /app

RUN apt update && \
    apt install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --target x86_64-unknown-linux-musl



FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/debug/webservice_rust .

CMD [ "./webservice_rust" ]
