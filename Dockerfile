FROM rust:1.50.0

ENV PATH /app/bin:$PATH
WORKDIR /build
COPY . /build
RUN cargo build --release

RUN mkdir -p /app/bin
RUN mkdir -p /app/conf
RUN cp /build/target/release/pinyin_tone_marks /app/bin
RUN cp /build/conf/log4rs.yaml /app/conf
WORKDIR /
RUN rm -rf /build

ENTRYPOINT ["pinyin_tone_marks"]