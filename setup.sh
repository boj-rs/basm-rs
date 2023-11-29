#!/bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --default-host x86_64-unknown-linux-gnu -y
sudo ln -s /home/codespace/.cargo/bin/cargo /bin/cargo
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb