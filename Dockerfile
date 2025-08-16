FROM rust:latest as build
WORKDIR /app
COPY . .
RUN cargo build -p demo --release
FROM debian:bookworm-slim
COPY --from=build /app/target/release/demo /usr/local/bin/demo
ENTRYPOINT ["demo"]
