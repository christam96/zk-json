# Use Alpine as the base image
# FROM alpine:latest
FROM rust:latest

# Install build essentials and OpenSSL development headers
RUN apt-get update && apt-get install -y build-essential libssl-dev git cmake ninja-build

# Update package index and install necessary dependencies
#RUN apk update && \
#    apk add --no-cache \
#    curl \
#    git \
#    build-base \
#    && rm -rf /var/cache/apk/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's binary directory to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Install cargo-binstall
RUN cargo install cargo-binstall

# Install cargo-risczero
RUN cargo binstall cargo-risczero -y

# Install risczero
# RUN cargo risczero install
RUN cargo risczero build-toolchain

# Clone the Git repository and run the application
RUN cd /root && \
    git clone https://github.com/christam96/zk-json.git && \
    cd zk-json && \
    cargo run

# Set the working directory for the container
WORKDIR /usr/src/app

# Define the default command to run when the container starts
CMD ["bash"]

