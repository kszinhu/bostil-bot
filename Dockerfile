#----------------
# Build stage
#----------------
FROM rust:1.76-alpine AS builder

ARG RUSTFLAGS="-C target-feature=-crt-static"
ARG APP=/usr/src/app
ARG TARGET_PLATFORM=x86_64-unknown-linux-musl
ARG CRATE_NAME=bostil-bot

# System dependencies
RUN apk add --no-cache \
  build-base cmake musl-dev pkgconfig openssl-dev \
  libpq-dev \
  curl git yt-dlp

WORKDIR ${APP}

RUN cargo new --bin ${CRATE_NAME}

WORKDIR ${APP}/${CRATE_NAME}

COPY Cargo.toml Cargo.lock ./
COPY app ./app
COPY core ./core

RUN cargo install diesel_cli --no-default-features --features postgres

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGET_PLATFORM} --mount=type=cache,target=/target,id=${TARGET_PLATFORM} \
  cargo build --release && \
  mv target/release/${CRATE_NAME} .

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry <<EOF
  set -e
  touch app/src/main.rs
  cargo build --release
EOF

CMD ["/target/release/${CRATE_NAME}"]

#----------------
# Runtime stage
#----------------
FROM alpine:3.19 AS runtime

ARG APP=/usr/src/app
ARG CRATE_NAME=bostil-bot

# System dependencies
RUN apk add --no-cache ca-certificates tzdata yt-dlp

# Copy the binary from the builder stage
# COPY --from=builder ${APP}/bostil-bot/target/release/bostil-bot ${APP}/bostil-bot
COPY --from=builder ${APP}/${CRATE_NAME} ${APP}/${CRATE_NAME}

WORKDIR ${APP}
RUN chmod +x ./${CRATE_NAME}

CMD [ "./${CRATE_NAME}" ]