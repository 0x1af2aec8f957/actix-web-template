FROM rust:latest as builder
LABEL author=Abner
LABEL version="1.0"
LABEL description="A Actix-Web Template"

ENV APP_NAME=actix-web-template
ENV WORKDIR=/usr/src/${APP_NAME}

WORKDIR $WORKDIR

COPY --chown=777 . .

RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y <extra-runtime-dependencies> rm -rf /var/lib/apt/lists/* # 额外的运行时依赖安装
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} /usr/local/bin/${APP_NAME}

EXPOSE 3000

CMD ["${APP_NAME}"]
