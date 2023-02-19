FROM rust:1.67

RUN apt-get update
RUN apt-get install --no-install-recommends --assume-yes protobuf-compiler

WORKDIR /app
COPY . .

ARG service
ENV service=$service

RUN cargo build --release --bin $service

CMD /app/target/release/$service
