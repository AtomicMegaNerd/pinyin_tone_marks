FROM rust:1.88.0-alpine3.22 as builder

ENV MUSL_DEV_VER 1.2.5-r10
RUN apk add --no-cache musl-dev==${MUSL_DEV_VER} \
    && rm -rf /var/cache/apk/*

WORKDIR /build
COPY . /build
RUN cargo build --release

FROM alpine:3.22

ENV GID 1001
ENV UID 1001
ENV USER dockeruser
ENV PATH=/app/bin:${PATH}

RUN mkdir -p /app/bin /app/conf \
    && adduser -D $USER \
    && chown $USER:$USER /app/bin

USER $USER
COPY --from=builder /build/target/release/pinyin_tone_marks /app/bin
WORKDIR /data

ENTRYPOINT ["pinyin_tone_marks"]
