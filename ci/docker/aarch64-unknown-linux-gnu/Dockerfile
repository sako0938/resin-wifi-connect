FROM dockcross/linux-arm64

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install -y libdbus-1-dev:arm64 libc6-dev:arm64

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN /root/.cargo/bin/rustup target add aarch64-unknown-linux-gnu

RUN ln -s /root/.cargo/bin/cargo /usr/local/sbin/cargo && \
    ln -s /root/.cargo/bin/rustc /usr/local/sbin/rustc && \
    ln -s /root/.cargo/bin/rustdoc /usr/local/sbin/rustdoc && \
    ln -s /root/.cargo/bin/rustup /usr/local/sbin/rustup

ENV PKG_CONFIG_ALLOW_CROSS 1

ENV PKG_CONFIG_LIBDIR /usr/lib/aarch64-linux-gnu/pkgconfig/

RUN ln -s /usr/bin/${CROSS_TRIPLE}-strip /usr/bin/stripbin

VOLUME /work
