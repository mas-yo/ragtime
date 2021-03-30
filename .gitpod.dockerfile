FROM gitpod/workspace-full:latest

RUN sudo apt-get update \
 && sudo apt-get install -y \
    telnet \
 && sudo rm -rf /var/lib/apt/lists/* \
 && rustup component add rustfmt \
 && rustup component add clippy
