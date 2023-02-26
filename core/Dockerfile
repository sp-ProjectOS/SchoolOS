## /* * * * * * * * * /
##  * SchoolOS Core *
##  * Dockerfile    *
## /* * * * * * * * * /

## Build the image with rust
FROM rust:1.54.0-buster as builder

## Run the following commands in the container
WORKDIR /usr/src/schoolos-core
RUN apt-get update && apt-get install -y libpq-dev
COPY . .
RUN cargo install --path .

## Build the image with the binary in alpine
FROM alpine:3.14.0
WORKDIR /usr/src/schoolos-core
USER root

## Copy the binary from the builder image
COPY --from=builder /usr/local/cargo/bin/schoolos-core /usr/local/bin/schoolos-core

## Run the binary
CMD ["schoolos-core"]