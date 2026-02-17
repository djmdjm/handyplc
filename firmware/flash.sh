#!/bin/sh

# Flash with STLinkII

set -xe
cargo build --release
cargo flash --chip STM32F411VETx
