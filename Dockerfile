FROM ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:latest

# add our foreign architecture and install our dependencies
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN dpkg --add-architecture armhf

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install sqlite3 -y

# add our linker search paths and link arguments
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS="-L /usr/lib/arm-linux-gnueabihf -C link-args=-Wl,-rpath-link,/usr/lib/arm-linux-gnueabihf $CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUSTFLAGS"