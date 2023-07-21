#----------------
# Build stage
#----------------
FROM rust:1.70.0-alpine3.17 as builder

# System dependencies, update pkg-config and libssl-dev
RUN apk update \
  && apk add --no-cache \
  build-base \
  curl \
  pkgconfig \
  openssl-dev \
  opus-dev \
  git \
  && rm -rf /var/cache/apk/*

WORKDIR /usr/src/app

RUN cargo new --bin bostil-bot

WORKDIR /usr/src/app/bostil-bot

COPY Cargo.toml ./Cargo.toml
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
RUN apk update \
  && apk add --no-cache ca-certificates tzdata \
  && rm -rf /var/cache/apk/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/bostil-bot/target/release/bostil-bot ${APP}/bostil-bot

RUN chmod +x ${APP}/bostil-bot
WORKDIR ${APP}

CMD [ "./bostil-bot" ]