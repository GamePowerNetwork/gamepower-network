FROM ubuntu:18.04 as builder

ARG PROFILE=release

RUN apt-get update && apt-get install -y \
    build-essential clang git\
    curl

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup update nightly
RUN rustup update stable
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

COPY . /gamepower
WORKDIR /gamepower

RUN cargo +nightly build --$PROFILE


# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the GamePower binary."
ARG PROFILE=release
COPY --from=builder /gamepower/target/$PROFILE/gamepower /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /gamepower gamepower && \
	mkdir -p /gamepower/.local/share && \
	mkdir /data && \
	chown -R gamepower:gamepower /data && \
	ln -s /data /gamepower/.local/share/gamepower && \
	rm -rf /usr/bin /usr/sbin

USER gamepower
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/gamepower"]