FROM rust:1.94 AS builder

WORKDIR /home/app/

RUN apt-get update && \
    apt-get install -y libpq-dev libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

COPY . ./

RUN cargo build --release

FROM alpine:3.23
RUN apk add --no-cache ca-certificates gcompat postgresql-libs libssl3 libgcc
COPY --from=builder /home/app/target/release/axum-server /usr/local/bin/axum-server
CMD ["axum-server"]
