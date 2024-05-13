FROM rust:1-bullseye

WORKDIR /app

COPY ${pwd} /app

run cargo build --release  

# ENTRYPOINT ["pwd"]
WORKDIR /app/target/release/


CMD ["./webservice_rust"]