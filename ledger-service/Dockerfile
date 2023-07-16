# Build Stage
FROM --platform=linux/amd64 rust:1.70-alpine3.17 as builder

RUN USER=root mkdir simplife
RUN sed -i 's/dl-cdn.alpinelinux.org/mirror.tuna.tsinghua.edu.cn/g' /etc/apk/repositories
RUN apk add musl-dev openssl openssl-dev pkgconfig upx git protobuf
WORKDIR ./simplife
COPY . ./
RUN cargo build --release --verbose --bin ledger-service && upx /simplife/target/release/ledger-service

FROM --platform=linux/amd64 alpine:3.17 as runtime
ENV TZ=Asia/Shanghai
RUN sed -i 's/dl-cdn.alpinelinux.org/mirror.tuna.tsinghua.edu.cn/g' /etc/apk/repositories
RUN apk update  \
    && apk add --no-cache vim tzdata bind-tools curl \
    && echo "${TZ}" > /etc/timezone \
    && ln -sf /usr/share/zoneinfo/${TZ} /etc/localtime \
    && rm -rf /var/cache/apk/*
COPY --from=builder /simplife/target/release/ledger-service /ledger-service
EXPOSE 27001
ENTRYPOINT ["/ledger-service"]