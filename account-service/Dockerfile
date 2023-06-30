# Build Stage
FROM rust:1.70-alpine3.17 as builder

RUN USER=root mkdir simplife
RUN apk add musl-dev openssl openssl-dev pkgconfig upx git protobuf
WORKDIR ./simplife
COPY . ./
RUN cargo build --release --verbose --bin account-service && upx /simplife/target/release/account-service

FROM alpine:3.17 as runtime
ENV TZ=Asia/Shanghai
RUN apk update  \
    && apk add --no-cache vim tzdata bind-tools curl \
    && echo "${TZ}" > /etc/timezone \
    && ln -sf /usr/share/zoneinfo/${TZ} /etc/localtime \
    && rm -rf /var/cache/apk/*
COPY --from=builder /simplife/target/release/account-service /account-service
EXPOSE 27001
ENTRYPOINT ["/account-service"]