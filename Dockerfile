FROM rust:1.56-alpine3.14 as builder

RUN apk add --no-cache musl-dev \
    && rm -rf /var/cache/apk/*

WORKDIR /build
COPY . /build
RUN cargo build --release

FROM alpine:3.14

ENV GID 1001
ENV UID 1001
ENV USER dockeruser
ENV PATH=/app/bin:${PATH}
ENV RUST_LOG=info

# Copy our built program over and 
RUN mkdir -p /app/bin /app/conf
COPY --from=builder /build/target/release/pinyin_tone_marks /app/bin
WORKDIR /data

ENTRYPOINT ["pinyin_tone_marks"]
