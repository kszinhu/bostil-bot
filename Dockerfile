#----------------
# Build stage
#----------------
FROM rust:1.76.0-alpine3.19 as builder

# System dependencies
RUN apk add --no-cache \
  build-base \
  cmake \
  musl-dev \
  curl \
  yt-dlp \
  pkgconfig \
  openssl-dev \
  git

WORKDIR /usr/src/app

RUN cargo new --bin bostil-bot

WORKDIR /usr/src/app/bostil-bot

COPY Cargo.toml ./Cargo.toml
COPY app ./app
COPY core ./core

# Build the dependencies
RUN cargo clean
RUN cargo build --release

# Remove the source code
RUN rm ./**/*.rs

ADD . ./

# Remove the target directory
RUN rm ./target/release/deps/bostil_bot*

# Build the application
RUN cargo build --release

#----------------
# Runtime stage
#----------------
FROM alpine:latest AS runtime

ARG APP=/usr/src/app

# System dependencies
RUN apk add --no-cache ca-certificates tzdata yt-dlp postgresql-dev

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/target/x86_64-unknown-linux-musl/release/bostil-bot ${APP}/bostil-bot

# Copy public files from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/app/public ${APP}/public

RUN chmod +x ${APP}/bostil-bot
WORKDIR ${APP}

CMD [ "./bostil-bot" ]