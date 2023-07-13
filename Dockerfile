# syntax=docker/dockerfile:1

FROM public.ecr.aws/docker/library/rust:1.70.0-buster


# WORKDIR /app
WORKDIR /app
COPY . /app

COPY . .

RUN cargo install --path .

RUN cargo build --release

RUN pwd
RUN ls -alh
RUN ls -alh /app/target
RUN ls -alh /app/target/release


RUN cp /app/target/release/container_limits /container_limits

CMD ["/container_limits"]
