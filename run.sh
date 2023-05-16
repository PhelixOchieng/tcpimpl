#!/bin/bash

# Build the app
cargo build --release
# Enables the app to create Virtual NIC withot being root
sudo setcap cap_net_admin=eip ./target/release/thunder
# Run app in background
./target/release/thunder &
# Capture its process id
pid=$!

# Add ip addresses to the network interface created by the app
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0

# Capture the app being closed by Ctrl-c and do some cleanup
trap "kill $pid" INT TERM
wait $pid
