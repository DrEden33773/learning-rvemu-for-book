FROM ubuntu:22.04

RUN sudo apt-get update && \
    sudo apt-get install -y build-essential && \
    sudo apt-get install -y curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env && \
    sudo apt-get install -y llvm-dev && \
    sudo apt-get clean && \
    sudo rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY . .

RUN cargo build && cargo test

CMD ["cargo", "test"]