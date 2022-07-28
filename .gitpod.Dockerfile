FROM gitpod/workspace-rust

RUN sudo apt-get install -y clang lld cmake

RUN rustup toolchain install beta nightly --profile default
RUN rustup target install wasm32-unknown-unknown
RUN rustup default beta

RUN cargo install cargo-deny cargo-udeps wasm-server-runner miniserve cargo-smart-release
