FROM rust:1.90-bullseye

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       build-essential \
       pkg-config \
       libsdl2-dev \
       libasound2-dev \
       libx11-dev \
       libxrandr-dev \
       libxcursor-dev \
       libxinerama-dev \
       libxi-dev \
       libgl1-mesa-dev \
       libudev-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp

ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

CMD ["bash"]
