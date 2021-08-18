###################
### BUILD STAGE ###
###################
FROM rust:1.53 as build

WORKDIR /app

# Add WebAssembly build target
RUN rustup target add wasm32-unknown-unknown

# Install SPA http server
RUN cargo install microserver

# Install Trunk
RUN cargo install --locked trunk

# Compile
COPY . .
RUN trunk build --release


########################
### PRODUCTION STAGE ###
########################
FROM debian:stable-slim

WORKDIR /dist

COPY --from=build /usr/local/cargo/bin/microserver /usr/local/bin/
COPY --from=build /app/dist/ .

CMD microserver -p 3000 /dist
