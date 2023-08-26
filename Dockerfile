FROM rust:1-alpine

RUN apk add musl-dev build-base pkgconfig

WORKDIR /usr/src/bullet-hell-game-server
COPY . .

RUN cargo install --path .

CMD ["bullet-hell-game-server"]