FROM balenalib/fincm3-debian:stretch

RUN install_packages build-essential curl file libglib2.0-dev gobject-introspection libgirepository1.0-dev python3-gi

ENV SDK_VERSION=0.2

ENV PATH=/root/.cargo/bin:$PATH

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /app

COPY fin-cli ./fin-cli
COPY fin-lib ./fin-lib
COPY fin-sdk ./fin-sdk
COPY include ./include
COPY Cargo.toml .
COPY Makefile .

RUN make

RUN strip target/release/fin

RUN strip target/release/libfin.so

WORKDIR /app/dist/sdk
RUN mv /app/target/release/libfin.so .
RUN mv /app/Fin-$SDK_VERSION.gir .
RUN mv /app/Fin-$SDK_VERSION.typelib .
RUN tar -czvf balena-fin-sdk-v$SDK_VERSION.0.tar.gz libfin.so Fin-$SDK_VERSION.gir Fin-$SDK_VERSION.typelib
RUN curl --upload-file balena-fin-sdk-v$SDK_VERSION.0.tar.gz https://transfer.sh/balena-fin-sdk-v$SDK_VERSION.0.tar.gz

WORKDIR /app/dist/cli
RUN mv /app/target/release/fin .
RUN tar -czvf balena-fin-cli-v$SDK_VERSION.0.tar.gz fin
RUN curl --upload-file balena-fin-cli-v$SDK_VERSION.0.tar.gz https://transfer.sh/balena-fin-cli-v$SDK_VERSION.0.tar.gz

CMD ["sleep", "infinity"]
