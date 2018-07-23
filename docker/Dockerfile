FROM "ubuntu:18.04" as builder

RUN apt-get update && apt-get install -y \
    curl \
    file \
    gcc \
    git \
    libssl-dev \
    make \
    pkg-config

RUN mkdir -p /opt && \
    git clone https://github.com/jvff/tdc-rust-async-presentation /opt/tdc-async-rust && \
    git clone https://github.com/koute/cargo-web /opt/cargo-web

RUN useradd -ms /bin/bash user
RUN chown -R user:user /opt /home/user
USER user

WORKDIR /opt/tdc-async-rust

RUN curl -sf -L https://sh.rustup.rs > /tmp/rustup.sh && \
    chmod +x /tmp/rustup.sh && \
    /tmp/rustup.sh -y

ENV PATH="${PATH}:/home/user/.cargo/bin"

RUN rustup toolchain install nightly && \
    rustup target add wasm32-unknown-unknown

RUN cd /opt/cargo-web && \
    git checkout 97cc32d42ca74a35fb67438efcad3d4f9e3ea068 && \
    cargo build --release

ENV PATH="/opt/cargo-web/target/release:${PATH}"

RUN make

FROM ubuntu:18.04

WORKDIR /opt/tdc-async-rust

COPY --from=builder /opt/tdc-async-rust/target/release/server /opt/tdc-async-rust/target/release/
COPY --from=builder /opt/tdc-async-rust/target/deploy/ /opt/tdc-async-rust/target/deploy/

EXPOSE 8080

CMD ["/opt/tdc-async-rust/target/release/server"]
