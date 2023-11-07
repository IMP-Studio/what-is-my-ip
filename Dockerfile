# base
FROM rust:1.73-bookworm AS base
WORKDIR /app
RUN update-ca-certificates
RUN apt update -y
RUN apt install build-essential -y
RUN apt install lld clang -y
RUN apt install make -y
RUN apt install git -y

# builder
FROM base AS builder
ENV USER=app
ENV UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" "${USER}"
WORKDIR /app
COPY . .
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release --locked
RUN strip -s target/release/what-is-my-ip

# final outcome
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/what-is-my-ip ./
EXPOSE 8080
CMD ["./what-is-my-ip"]
