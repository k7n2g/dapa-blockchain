FROM lukemathwalker/cargo-chef:0.1.66-rust-1.77.1-slim-bookworm AS chef

ENV BUILD_DIR /tmp/xelis-build

RUN mkdir -p $BUILD_DIR
WORKDIR $BUILD_DIR

# ---

FROM chef AS planner

ARG app

COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin $app

# ---

FROM chef AS builder

ARG app
ARG commit_hash

COPY --from=planner /tmp/xelis-build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin $app

COPY Cargo.toml Cargo.lock ./
COPY dapa_common ./dapa_common
COPY $app ./$app

RUN DAPA_COMMIT_HASH=${commit_hash} cargo build --release --bin $app

# ---

FROM gcr.io/distroless/cc-debian12

ARG app

ENV APP_DIR /var/run/dapa
ENV DATA_DIR $APP_DIR/data
ENV BINARY $APP_DIR/dapa

LABEL org.opencontainers.image.authors="Slixe <slixeprivate@gmail.com>"

COPY --from=builder /tmp/dapa-build/target/release/$app $BINARY

WORKDIR $DATA_DIR

ENTRYPOINT ["/var/run/dapa/dapa"]
