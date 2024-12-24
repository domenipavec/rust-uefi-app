#!/bin/bash

set -e



cargo build --target x86_64-unknown-uefi

uefi-run target/x86_64-unknown-uefi/debug/rust-uefi-app.efi -d -- -net nic,model=virtio,macaddr=52:54:00:00:00:01 -net bridge,br=br0
