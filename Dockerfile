FROM debian:bookworm-slim AS build
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config \
    openssl libssl-dev python3.11 python3.11-dev && \
    update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.11 1 && \
    update-alternatives --set python3 /usr/bin/python3.11 && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install 1.91.1 && rustup default 1.91.1

ENV PYO3_PYTHON=/usr/bin/python3.11

WORKDIR /mcep
COPY . ./
RUN make release

FROM debian:bookworm-slim AS image
EXPOSE 8080

RUN apt-get update && \
    apt-get install -y python3.11 libpython3.11 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /mcep
RUN mkdir config
COPY --from=build /mcep/target/release/mcep .
COPY --from=build /mcep/config/*toml ./config
CMD ["./mcep"]