#!/bin/bash

echo 'eval "$(starship init bash)"' >> ~/.bashrc
mkdir -p ~/.config
cp .devcontainer/starship.toml ~/.config

# Make updateContent.sh executable
chmod +x .devcontainer/updateContent.sh

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh