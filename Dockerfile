FROM rust:1.82.0-bookworm as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin fromis

FROM debian:bookworm-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/fromis /usr/local/bin/fromis

RUN apt-get update \
    && apt-get install -y --no-install-recommends openssl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group fromis
USER fromis
WORKDIR /home/fromis

LABEL org.opencontainers.image.source=https://github.com/m1sk9/fromis

ENV RUST_LOG=fromis=info

ENTRYPOINT [ "sh", "-c", "fromis" ]
