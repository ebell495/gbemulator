FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /gbemulator
WORKDIR /gbemulator/fuzz
RUN cargo build 

FROM debian:bookworm
COPY --from=builder /gbemulator/fuzz/target/debug/lib_gbemulation-fuzz /