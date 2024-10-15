FROM docker.io/clux/muslrust:stable as builder
RUN rustup install stable --profile minimal
WORKDIR /build
RUN git clone https://github.com/Warthunder-Open-Source-Foundation/wt_event_handler.git
WORKDIR /build/wt_event_handler
RUN cargo build --release

FROM alpine
WORKDIR /running
COPY --from=builder /build/wt_event_handler/target/x86_64-unknown-linux-musl/release/wt_event_handler .
RUN mkdir assets
COPY discord_token.json ./assets
CMD ./wt_event_handler 1