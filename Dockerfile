FROM rust:1.74 as dev

RUN apt-get update
RUN apt-get -y install --no-install-recommends  \
    libboost-dev libboost-program-options-dev  \
    libboost-system-dev libboost-thread-dev  \
    libboost-math-dev libboost-test-dev libboost-python-dev \
    zlib1g-dev cmake wget build-essential
RUN wget --no-check-certificate https://www.python.org/ftp/python/3.10.13/Python-3.10.13.tgz && \
    tar -xvf Python-3.10.13.tgz && \
    cd Python-3.10.13 && \
    ./configure --enable-optimizations --enable-shared --prefix=/usr && \
    make && \
    make install && \
    ls -la /usr/local/lib | grep libpython3.10

WORKDIR /mcep
COPY . ./
#RUN make build

#FROM ubuntu:22.04
#FROM ubuntu:22.04 AS dev
#WORKDIR /mcep
#
#RUN apt-get update && \
#    apt-get install wget build-essential software-properties-common -y
#
#RUN apt-get update && \
#    add-apt-repository ppa:deadsnakes/ppa -y && \
#    apt-get install python3.10
#
#COPY . ./
#RUN #make build