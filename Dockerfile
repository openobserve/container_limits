# syntax=docker/dockerfile:1

FROM public.ecr.aws/docker/library/rust:1.70.0-buster


WORKDIR /app

COPY . .

RUN cargo install --path .

RUN cargo build --release

COPY target/release/container_limits /container_limits

CMD ["/container_limits"]
