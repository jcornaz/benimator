FROM gitpod/workspace-rust

# Repository for github-cli
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
RUN sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null

RUN sudo apt-get update
RUN sudo apt-get upgrade -y
RUN sudo apt-get install -y clang lld cmake gh

RUN rustup toolchain install --profile default beta nightly
RUN rustup default beta
RUN rustup target install wasm32-unknown-unknown

RUN cargo install cargo-deny cargo-udeps wasm-server-runner miniserve cargo-smart-release
