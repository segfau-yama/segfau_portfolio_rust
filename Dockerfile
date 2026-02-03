FROM rust:1.93

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src

RUN rustup toolchain install stable
RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli


COPY Cargo.toml Cargo.lock Dioxus.toml tailwind.css ./
COPY assets ./assets
COPY src ./src

EXPOSE 8080

CMD ["dx", "serve"]