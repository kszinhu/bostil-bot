#----------------
# Build stage
#----------------
FROM rust:1.71.0-alpine3.17 as builder

# System dependencies, update pkg-config and libssl-dev
RUN apk add --no-cache \
  build-base \
  musl-dev \
  curl \
  ffmpeg \
  youtube-dl \
  pkgconfig \
  openssl-dev \
  opus \
  opus-dev \
  git

WORKDIR /usr/src/app

RUN cargo new --bin bostil-bot

WORKDIR /usr/src/app/bostil-bot

COPY Cargo.toml ./Cargo.toml
COPY public ./public
COPY src ./src

# Build and cache the dependencies
RUN cargo fetch \
  && cargo build --release \
  && rm src/**/*.rs

ADD . ./

# Remove the target dependencies
RUN rm ./target/release/deps/bostil_bot*

# Build the application
RUN cargo build --release

#----------------
# Runtime stage
#----------------
FROM ubuntu:22.04 as runtime

ARG APP=/usr/src/app

# System dependencies
# RUN apk add --no-cache musl-dev libgcc ca-certificates tzdata ffmpeg youtube-dl opus opus-dev curl
RUN apt update \
  && apt install ffmpeg youtube-dl libopus-dev ca-certificates -y \
  && rm -r /var/cache/apt

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/target/release/bostil-bot ${APP}/bostil-bot

# Copy public files from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/public ${APP}/public

RUN chmod +x ${APP}/bostil-bot
WORKDIR ${APP}

CMD [ "./bostil-bot" ]