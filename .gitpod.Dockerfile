FROM gitpod/workspace-rust

RUN rustup target install wasm32-unknown-unknown
RUN rustup component add clippy
