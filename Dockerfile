FROM rust:1.52.1-alpine3.13 as builder

RUN apk add --no-cache musl-dev \
    && rm -rf /var/cache/apk/*

WORKDIR /build
COPY . /build
RUN cargo build --release

FROM alpine:3.13

ENV PATH /app/bin:$PATH
RUN mkdir -p /app/bin
RUN mkdir -p /app/conf
COPY --from=builder /build/target/release/pinyin_tone_marks /app/bin
COPY --from=builder /build/conf/log4rs.yaml /app/conf

ENTRYPOINT ["pinyin_tone_marks"]
