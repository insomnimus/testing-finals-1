FROM rust:buster AS build

WORKDIR app

COPY . .

ENV RUSTFLAGS="--cfg blackbox_tests"
RUN cargo build
RUN cargo test

FROM golang:1.17.5-buster AS blackbox

WORKDIR tests
COPY ./blackbox-tests .
RUN go build -o test .

COPY --from=build /app/target/debug/chat /bin/chat

RUN chmod +x ./test /bin/chat

CMD ["./test", "/bin/chat"]
