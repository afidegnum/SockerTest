# get the base image
FROM rust:1.73-slim-bullseye AS build

# install build dependencies
RUN apt update \
  && apt install -y --no-install-recommends lsb-release apt-transport-https pkg-config \
  build-essential curl wget libudev-dev libssl-dev strace

# vars
ENV PERSEUS_VERSION=0.4.2 \
  PERSEUS_SIZE_OPT_VERSION=0.1.7 \
  ESBUILD_VERSION=0.19.5 \
  BINARYEN_VERSION=116 \
  PERSEUS_HOST=0.0.0.0 \
  PERSEUS_PORT=8080 \
  RUST_BACKTRACE=1
# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# retrieve the src dir
# RUN curl https://codeload.github.com/framesurge/perseus-size-opt/tar.gz/main | tar -xz --strip=2 perseus-size-opt-main/examples/simple

# go to src dir
# WORKDIR /app/simple

# install perseus-cli
RUN cargo install perseus-cli --version $PERSEUS_VERSION

# clean and prep app
RUN perseus clean

# # specify deps in app config
# RUN sed -i s"/perseus = .*/perseus = \"${PERSEUS_VERSION}\"/" ./Cargo.toml \
#   && sed -i s"/perseus-size-opt = .*/perseus-size-opt = \"${PERSEUS_SIZE_OPT_VERSION}\"/" ./Cargo.toml \
#   && cat ./Cargo.toml

# modify lib.rs
# RUN sed -i s'/SizeOpts::default()/SizeOpts { wee_alloc: true, lto: true, opt_level: "s".to_string(), codegen_units: 1, enable_fluent_bundle_patch: false, }/' ./src/lib.rs \
#   && cat ./src/lib.rs

# run plugin(s) to adjust app
# RUN perseus tinker \
#   && cat .perseus/Cargo.toml \
#   && cat ./src/lib.rs

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true
COPY Cargo.toml Cargo.toml

# copy app files
COPY src src



# deploy app
RUN perseus deploy

# go back to app dir
# WORKDIR /app

# download and unpack esbuild
RUN curl -O https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-${ESBUILD_VERSION}.tgz \
  && tar xf linux-x64-${ESBUILD_VERSION}.tgz \
  && ./package/bin/esbuild --version

# run esbuild against bundle.js
RUN ./package/bin/esbuild ./pkg/dist/pkg/perseus_engine.js --minify --target=es6 --outfile=./pkg/dist/pkg/perseus_engine.js --allow-overwrite \
  && ls -lha ./pkg/dist/pkg

# download and unpack binaryen
RUN wget -nv https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar xf binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt --version

# run wasm-opt against bundle.wasm
RUN ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt -Os ./pkg/dist/pkg/perseus_engine_bg.wasm -o ./pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./pkg/dist/pkg

# prepare deployment image
FROM debian:bullseye-slim AS final

WORKDIR /app

USER 1000

# indicate what port the server is running on

COPY --from=build /app/pkg /app/

ENV PERSEUS_HOST=0.0.0.0 \
  PERSEUS_PORT=8080 \
  RUST_BACKTRACE=1

#  EXPOSE 8080

# CMD ["./server"]
CMD ["strace", "-f", "./server"]
