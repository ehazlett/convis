FROM ubuntu:21.04 as rust
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y curl build-essential cmake libprotobuf-c-dev protobuf-c-compiler
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

FROM rust as build
COPY . /app
WORKDIR /app
RUN bash -c "source /root/.cargo/env && cargo build --release"

FROM scratch as binary
COPY --from=build /app/target/release/convis /convis

FROM ubuntu:21.04
RUN apt-get update && apt-get install -y strace curl
COPY --from=build /app/target/release/convis /usr/local/bin/convis
COPY .docker/entrypoint.sh /usr/local/bin/entrypoint.sh
CMD ["/usr/local/bin/entrypoint.sh"]
