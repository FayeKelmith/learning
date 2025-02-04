FROM rust:1.82 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/advanced /usr/local/bin/advanced
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["advanced"]