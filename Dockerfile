### Builder
FROM rust:1 as builder

WORKDIR /usr/src/nerfthis-tools

COPY . .

RUN cargo install --path .


### Runner
FROM debian:buster as runner

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/local/cargo/bin/nerfthis-tools /usr/local/bin/nerfthis-tools
COPY --from=builder /usr/src/nerfthis-tools/templates ./templates
COPY --from=builder /usr/src/nerfthis-tools/Rocket.toml .

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["nerfthis-tools"]

