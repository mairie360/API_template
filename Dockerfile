FROM rust:1.98-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app

# change api name
COPY --from=builder /usr/src/app/target/release/api_template /app/template-api

# change api name
CMD ["/app/template-api"]
