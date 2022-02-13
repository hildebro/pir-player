FROM rustembedded/cross:arm-unknown-linux-gnueabihf

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install libasound-dev -y && \
    apt-get install libasound-dev:armhf -y