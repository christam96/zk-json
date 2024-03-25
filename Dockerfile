FROM python:3.12.2-bookworm

# Install dependencies if needed
RUN cd /root \
&& git clone https://github.com/christam96/zk-json.git \
&& cd zk-json \
&& cargo run

