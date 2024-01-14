# NB: This is not a production-grade Dockerfile.

#################
## build stage ##
#################
FROM rust:1-slim-bullseye AS builder



WORKDIR /code

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build downloading the index
# which can take a long time within the docker context. Docker will cache it.
COPY Cargo.toml Cargo.toml
# RUN cargo fetch

# copy app files
COPY src src

# compile app
RUN cargo build --release

###############
## run stage ##
###############

FROM debian:bullseye-slim
# ARG UID=1000
# RUN adduser \
#     --disabled-password \
#     --gecos "" \
#     --home "/nonexistent" \
#     --shell "/sbin/nologin" \
#     --no-create-home \
#     --uid "${UID}" \
#     afidegnum


RUN apt update \
    && apt install -y --no-install-recommends lsb-release apt-transport-https pkg-config \
    build-essential curl wget iproute2 netcat-openbsd socat libudev-dev libssl-dev iputils-ping


# USER 1000

WORKDIR /app

# copy server binary from build stage
COPY --from=builder /code/target/release/hypertest hypertest


# RUN mkdir sock

# RUN USER=root chown -R afidegnum:afidegnum sock/

# RUN USER=root chmod -R 655 sock/
# # set user to non-root unless root is required for your app

# indicate what port the server is running on
# EXPOSE 4400

# run server
RUN ls -l -a /tmp

RUN ls -l -a /tmp
RUN ls -l -a /tmp
RUN ls -l -a /tmp
ENV RUST_BACKTRACE=1
CMD [ "/app/hypertest" ]
