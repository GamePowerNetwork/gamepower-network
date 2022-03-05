FROM ubuntu:18.04
RUN apt-get update && apt-get install -y \
    build-essential clang git\
    curl

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup update nightly
RUN rustup update stable
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
COPY . /gamepower-wallet-template
WORKDIR /gamepower-wallet-template
RUN cargo +nightly build --release
EXPOSE 30333 9933 9944 9615
CMD cargo +nightly run --release -- --dev --tmp --ws-external