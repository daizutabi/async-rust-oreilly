#!/bin/bash

# Install required packages for OpenSSL and pkg-config
sudo apt-get update
sudo apt-get install -y \
    pkg-config \
    libssl-dev
