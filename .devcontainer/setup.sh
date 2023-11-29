#!/bin/bash
apt-get -y update
apt-get -y install curl git gnupg2 jq sudo vim build-essential openssl python3
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --default-host x86_64-unknown-linux-gnu -y
sudo ln -s /home/codespace/.cargo/bin/cargo /bin/cargo