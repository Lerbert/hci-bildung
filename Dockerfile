FROM node:latest as vue-build

WORKDIR /app/vue
RUN mkdir -p /app/templates/sheet
COPY vue/package*.json ./
RUN npm install

COPY ./vue .

RUN npm run build

FROM rust:latest as rust-build

RUN user=root cargo new --bin hci-bildung
WORKDIR /hci-bildung
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./migrations ./migrations
COPY ./src ./src

RUN rm ./target/release/deps/hci_bildung*
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /hci-bildung

COPY --from=vue-build /app/vue_dist ./vue_dist
COPY --from=rust-build /hci-bildung/target/release/hci-bildung .

COPY ./deployment/launch_server.sh ./launch.sh
COPY ./Rocket.toml ./Rocket.toml
COPY ./assets ./assets
COPY ./templates ./templates
COPY --from=vue-build /app/templates ./templates

EXPOSE 8000
CMD ["./launch.sh"]
