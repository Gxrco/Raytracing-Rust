#!/bin/bash

cargo build --release && ./target/release/$(basename $(pwd))
