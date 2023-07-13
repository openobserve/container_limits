# syntax=docker/dockerfile:1

FROM public.ecr.aws/docker/library/rust:1.70.0-buster


# WORKDIR /app
WORKDIR /app
COPY . /app

COPY . .
RUN pwd
RUN ls -la

RUN cargo install --path .

RUN cargo build --release

COPY /app/target/release/container_limits /container_limits

CMD ["/container_limits"]
