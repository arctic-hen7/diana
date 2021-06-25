#!/bin/bash

# This is a simple build script that sets up the serverless function for deployment to Netlify

cargo build --release --target x86_64-unknown-linux-musl
cp ../target/x86_64-unknown-linux-musl/release/serverless functions

# Unfortunately, we can't run `strip` on the final binary to reduce its size becuase then Netlify ignores it completely!
