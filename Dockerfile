FROM ubuntu:22.04 AS build

RUN apt-get update && \
    apt-get install -y curl build-essential software-properties-common pkg-config  \
    openssl libssl-dev -y && \
    apt-get update

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install 1.75.0

RUN apt-get update && \
    add-apt-repository ppa:deadsnakes/ppa -y && \
    apt-get update &&  \
    apt-get install -y python3.10-dev

WORKDIR /mcep
COPY . ./
RUN make build

FROM ubuntu:22.04 AS image
EXPOSE 8080
RUN apt-get update && \
    apt-get install -y software-properties-common && \
    add-apt-repository ppa:deadsnakes/ppa -y && \
    apt-get update &&  \
    apt-get install -y python3.10-dev
WORKDIR /mcep
RUN mkdir config
COPY --from=build /mcep/target/debug/mcep .
COPY --from=build /mcep/Rocket.toml .
COPY --from=build /mcep/config/*toml ./config
CMD ["./mcep"]