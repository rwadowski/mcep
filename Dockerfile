FROM debian:bookworm-slim AS build
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y curl build-essential pkg-config \
    openssl libssl-dev python3.11 python3.11-dev && \
    update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.11 1 && \
    update-alternatives --set python3 /usr/bin/python3.11 && \
    rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install 1.97.1 && rustup default 1.97.1

ENV PYO3_PYTHON=/usr/bin/python3.11

WORKDIR /mcep

COPY frontend/package*.json ./frontend/
RUN cd frontend && npm ci

COPY . ./
RUN cd frontend && npm run build
RUN make release

FROM debian:bookworm-slim AS image
EXPOSE 8080

RUN apt-get update && \
    apt-get install -y python3.11 libpython3.11 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /mcep
RUN mkdir -p config frontend/dist
COPY --from=build /mcep/target/release/mcep .
COPY --from=build /mcep/config/*toml ./config/
COPY --from=build /mcep/frontend/dist/ ./frontend/dist/
CMD ["./mcep"]
