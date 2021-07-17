###################
### BUILD STAGE ###
###################
FROM rust:1.53 as build

WORKDIR /app

COPY . .

# Add WebAssembly build target
RUN rustup target add wasm32-unknown-unknown

# Install SPA http server
RUN cargo install microserver

# Install Trunk with dependencies
RUN apt-get update && apt-get install -y binaryen
RUN cargo install wasm-bindgen-cli # TODO: eventually Trunk will automatically install this
RUN cargo install --locked trunk

# Compile
RUN trunk build --release


########################
### PRODUCTION STAGE ###
########################
FROM debian:stable-slim

WORKDIR /dist

COPY --from=build /usr/local/cargo/bin/microserver /usr/local/bin/
COPY --from=build /app/dist/ .

CMD microserver -p 3000 /dist
