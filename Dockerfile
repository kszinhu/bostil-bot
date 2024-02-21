#----------------
# Build stage
#----------------
FROM rust:1.76-alpine AS builder

ARG RUSTFLAGS="-C target-feature=-crt-static"
ARG APP=/usr/app
ARG CRATE_NAME=bostil-bot

# System dependencies
RUN apk add --no-cache \
  build-base cmake musl-dev pkgconfig openssl-dev \
  libpq-dev \
  curl git yt-dlp

WORKDIR ${APP}

# Build dependencies
# create a dummy source file to cache the dependencies
COPY Cargo.toml ./
COPY app/Cargo.toml ./app/
COPY core/Cargo.toml ./core/
RUN mkdir -p ./app/src && echo "fn main() {println!(\"if you see this, the build broke\")}" > ./app/src/main.rs
RUN mkdir -p ./core/src && echo "" > ./core/src/lib.rs
RUN cargo build --release

# Replace with real source code
RUN rm -f ./app/src/main.rs
COPY app ./app
COPY core ./core

# Break the Cargo cache
RUN touch ./app/src/main.rs
RUN touch ./core/src/lib.rs

# Build the project
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/src/app/target \
  cargo build --release

RUN strip target/release/${CRATE_NAME}

#----------------
# Runtime stage
#----------------
FROM alpine:3.19 AS runtime

ARG APP=/usr/app
ARG CRATE_NAME=bostil-bot

# System dependencies
RUN apk add --no-cache ca-certificates tzdata yt-dlp

WORKDIR ${APP}

# Copy the binary from the builder stage
COPY --from=builder ${APP}/target/release/${CRATE_NAME} ./

# Run the application
CMD ["./bostil-bot"]
