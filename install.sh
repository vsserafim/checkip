#!/bin/bash -e

cargo build --release

# Install checkip release build into /usr/local/bin
sudo install -m 755 ./target/release/checkip /usr/local/bin/checkip
