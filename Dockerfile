ARG APP_NAME=actix-web-template # 利用 --build-arg 传递多阶段共享变量参数

FROM rust:latest as builder
ARG APP_NAME
ENV WORKDIR=/usr/src/${APP_NAME}

WORKDIR $WORKDIR

COPY --chown=777 . .

RUN cargo install --path .

FROM debian:bullseye-slim
ARG APP_NAME
LABEL author=Abner
LABEL version="1.0"
LABEL description="A Actix-Web Template"

ENV WORKFILE=/usr/local/bin/${APP_NAME}

# RUN apt-get update && apt-get install -y <extra-runtime-dependencies> rm -rf /var/lib/apt/lists/* # 额外的运行时依赖安装
COPY --from=builder /usr/local/cargo/bin/${APP_NAME} ${WORKFILE}

EXPOSE 3000

ENTRYPOINT ["${WORKFILE}"]
