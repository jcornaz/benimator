FROM gitpod/workspace-rust

RUN rustup target install wasm32-unknown-unknown
RUN cargo install wasm-server-runner
RUN rustup component add clippy
