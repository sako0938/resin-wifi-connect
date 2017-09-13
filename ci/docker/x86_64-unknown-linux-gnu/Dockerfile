FROM dockcross/linux-x64

RUN apt-get update && \
    apt-get install -y libdbus-1-dev libc6-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN ln -s /root/.cargo/bin/cargo /usr/local/sbin/cargo && \
    ln -s /root/.cargo/bin/rustc /usr/local/sbin/rustc && \
    ln -s /root/.cargo/bin/rustdoc /usr/local/sbin/rustdoc && \
    ln -s /root/.cargo/bin/rustup /usr/local/sbin/rustup

RUN ln -s /usr/bin/${CROSS_TRIPLE}-strip /usr/bin/stripbin

VOLUME /work
