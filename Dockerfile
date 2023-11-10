FROM rust:lastest as builder
LABEL author=Abner
LABEL version="1.0"
LABEL description="A Actix-Web Template"

ENV APP_NAME=actix-web-template
ENV WORKDIR=/usr/src/${APP_NAME}
COPY --chown=777 . .

WORKDIR $WORKDIR

RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/${APP_NAME}

CMD ["${APP_NAME}"]