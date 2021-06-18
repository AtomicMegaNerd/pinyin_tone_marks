FROM rust:1.53l-alpine3.13 as builder

RUN apk add --no-cache musl-dev \
    && rm -rf /var/cache/apk/*

WORKDIR /build
COPY . /build
RUN cargo build --release

FROM alpine:3.13

ENV GID 1001
ENV UID 1001
ENV USER dockeruser
ENV PATH=/app/bin:${PATH}

# We are going to run as a non-root user
RUN apk add --no-cache shadow
RUN groupadd -g ${GID} ${USER} && useradd -g ${GID} -u ${UID} -m ${USER}

# Copy our built program over and 
RUN mkdir -p /app/bin /app/conf
COPY --from=builder /build/target/release/pinyin_tone_marks /app/bin
COPY --from=builder /build/conf/log4rs.yaml /app/conf
RUN chmod a+rx /app/bin/pinyin_tone_marks && chmod a+r /app/conf/log4rs.yaml

USER ${USER}

ENTRYPOINT ["pinyin_tone_marks"]
