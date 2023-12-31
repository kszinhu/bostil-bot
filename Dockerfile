#----------------
# Build stage
#----------------
FROM rust:1.71.0-alpine3.17 as builder

# System dependencies
RUN apk add --no-cache \
  build-base \
  cmake \
  musl-dev \
  curl \
  ffmpeg \
  youtube-dl \
  pkgconfig \
  openssl-dev \
  git

WORKDIR /usr/src/app

RUN cargo new --bin bostil-bot

WORKDIR /usr/src/app/bostil-bot

COPY Cargo.toml ./Cargo.toml
COPY public ./public
COPY src ./src

# Build the dependencies
RUN cargo clean
RUN cargo build --release

# Remove the source code
RUN rm src/**/*.rs

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
RUN apk add --no-cache ca-certificates tzdata youtube-dl ffmpeg

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/target/release/bostil-bot ${APP}/bostil-bot

# Copy public files from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/public ${APP}/public

RUN chmod +x ${APP}/bostil-bot
WORKDIR ${APP}

CMD [ "./bostil-bot" ]