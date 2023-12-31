FROM ubuntu:22.04

RUN apt-get update && \
    apt-get install -y build-essential && \
    apt-get install -y curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env && \
    apt-get install -y llvm-dev clang lld && \
    update-alternatives --install /usr/bin/ld ld /usr/bin/lld 100 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app

COPY . .

RUN cargo build && cargo test

CMD ["cargo", "test"]