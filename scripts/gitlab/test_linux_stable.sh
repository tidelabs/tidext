#!/usr/bin/env bash
set -eux

time cargo test --all-targets --workspace
