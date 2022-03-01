FROM node:latest as vue-build

WORKDIR /app/vue
COPY vue/package*.json ./
RUN npm install

COPY ./vue .

RUN npm run build

FROM rust:1.59 as rust-build

RUN user=root cargo new --bin editor-server
WORKDIR /editor-server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/editor_server*
RUN cargo build --release

FROM debian:buster-slim

WORKDIR /editor-server

COPY --from=vue-build /app/vue_dist ./vue_dist
COPY --from=rust-build /editor-server/target/release/editor-server .

COPY ./deployment/launch_server.sh ./launch.sh
COPY ./Rocket.toml ./Rocket.toml
COPY ./assets ./assets
COPY ./templates ./templates

EXPOSE 8000
CMD ["./launch.sh"]
