# What is My IP

Service to check your IP address.
to run it you mus have `rustup`

## Run Locally

```bash
cargo watch -x run
```

## build docker images

```bash
docker build . -t what-is-my-ip
```

## Run on Production

```bash
docker run ghcr.io/IMP-Studio/what-is-my-ip
```