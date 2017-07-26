FROM dockcross/linux-armv7

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y libdbus-1-dev:armhf libc6-dev:armhf

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN /root/.cargo/bin/rustup target add armv7-unknown-linux-gnueabihf

RUN ln -s /root/.cargo/bin/cargo /usr/local/sbin/cargo && \
    ln -s /root/.cargo/bin/rustc /usr/local/sbin/rustc && \
    ln -s /root/.cargo/bin/rustdoc /usr/local/sbin/rustdoc && \
    ln -s /root/.cargo/bin/rustup /usr/local/sbin/rustup

ENV PKG_CONFIG_ALLOW_CROSS 1

ENV PKG_CONFIG_LIBDIR /usr/lib/arm-linux-gnueabihf/pkgconfig/

VOLUME /work
