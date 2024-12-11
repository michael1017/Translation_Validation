# Use Ubuntu as the base image
FROM ubuntu:22.04

WORKDIR /app

RUN apt-get update && apt-get install -y \
    curl wget build-essential python3 python3-pip bash-completion \
    && curl https://sh.rustup.rs -sSf | bash -s -- -y \
    && . $HOME/.cargo/env \
    && cargo install --locked kani-verifier \
    && cargo kani setup \
    && wget https://github.com/diffblue/cbmc/releases/download/cbmc-6.4.1/ubuntu-22.04-cbmc-6.4.1-Linux.deb \
    && dpkg -i ubuntu-22.04-cbmc-6.4.1-Linux.deb \
    && rm ubuntu-22.04-cbmc-6.4.1-Linux.deb



