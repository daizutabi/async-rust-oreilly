FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu24.04

# Install packages required for building Rust applications
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
