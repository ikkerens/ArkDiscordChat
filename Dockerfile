# Set up rust build environment
FROM rust:latest
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

# Build
WORKDIR /usr/src/arkdiscordchat
COPY . .
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Create a minimal docker file with only the resulting binary
FROM scratch
COPY --from=0 /usr/local/cargo/bin/arkdiscordchat .
USER 1000
CMD ["./arkdiscordchat"]