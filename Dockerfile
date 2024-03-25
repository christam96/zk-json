# Use Ubuntu as the base image
FROM ubuntu:latest

# Update package index and install necessary dependencies
RUN apt-get update && \
    apt-get install -y \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo's binary directory to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Verify installation
RUN cargo --version

RUN cd /root \
&& git clone https://github.com/christam96/zk-json.git \
&& cd zk-json \
&& cargo run

# Define the default command to run when the container starts
CMD ["bash"]

