FROM dockcross/linux-x86

RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -y libdbus-1-dev:i386 libc6-dev:i386

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN /root/.cargo/bin/rustup target add i686-unknown-linux-gnu

RUN ln -s /root/.cargo/bin/cargo /usr/local/sbin/cargo && \
    ln -s /root/.cargo/bin/rustc /usr/local/sbin/rustc && \
    ln -s /root/.cargo/bin/rustdoc /usr/local/sbin/rustdoc && \
    ln -s /root/.cargo/bin/rustup /usr/local/sbin/rustup

ENV PKG_CONFIG_ALLOW_CROSS 1

ENV PKG_CONFIG_LIBDIR /usr/lib/i386-linux-gnu/pkgconfig/

RUN ln -s /usr/bin/x86_64-linux-gnu-strip /usr/bin/stripbin

VOLUME /work
