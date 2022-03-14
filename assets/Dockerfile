FROM archlinux
RUN pacman -Syu --noconfirm rustup git base-devel
RUN pacman --noconfirm -Scc
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu
ADD "https://www.random.org/cgi-bin/randbyte?nbytes=10&format=h" skipcache
RUN git clone https://github.com/Warthunder-Open-Source-Foundation/wt_event_handler.git
WORKDIR ./wt_event_handler
RUN cargo build --release

COPY discord_token.json ./assets