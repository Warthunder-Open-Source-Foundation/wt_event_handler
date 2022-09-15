FROM archlinux
RUN pacman -Syu --noconfirm rustup git base-devel
RUN pacman --noconfirm -Scc
ADD "https://www.random.org/cgi-bin/randbyte?nbytes=10&format=h" skipcache
RUN rustup default nightly
RUN git clone https://github.com/Warthunder-Open-Source-Foundation/wt_event_handler.git
WORKDIR ./wt_event_handler
RUN cargo build --release
RUN mv target/release/wt_event_handler wt_event_handler
RUN rm -rf target

COPY discord_token.json ./assets
