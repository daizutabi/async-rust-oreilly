#!/usr/bin/env bash

SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cargo build --release -p server_bin -p connection_bin -p process_spawner
cp ../../../../target/release/connection_bin .
cp ../../../../target/release/server_bin .
cp ../../../../target/release/process_spawner .
