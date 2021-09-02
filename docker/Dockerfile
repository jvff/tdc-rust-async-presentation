FROM "debian:11" as builder

RUN apt-get update && apt-get install -y \
    curl \
    file \
    g++ \
    git \
    libssl-dev \
    make \
    pkg-config

RUN mkdir -p /opt && \
    git clone https://github.com/jvff/tdc-rust-async-presentation /opt/async-rust

RUN useradd -ms /bin/bash user
RUN chown -R user:user /opt /home/user
USER user

WORKDIR /opt/async-rust

RUN curl -sf -L https://sh.rustup.rs > /tmp/rustup.sh && \
    chmod +x /tmp/rustup.sh && \
    /tmp/rustup.sh -y

ENV PATH="${PATH}:/home/user/.cargo/bin"

RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk

RUN make

FROM debian:11

WORKDIR /opt/async-rust

COPY --from=builder /opt/async-rust/target/release/server /opt/async-rust/
COPY --from=builder /opt/async-rust/dist/ /opt/async-rust/dist

EXPOSE 8182

CMD ["/opt/async-rust/server"]
