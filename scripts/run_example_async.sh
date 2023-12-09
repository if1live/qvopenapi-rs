#!/bin/bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR/.."

if [ -f ".env.local" ]; then
    source .env.local
fi

if [ ! -f "./target/i686-pc-windows-gnu/release/wmca.dll" ]
then
    $SCRIPT_DIR/download_dll.sh
fi

cargo rustc -p qvopenapi-async --example connect_async --release --target i686-pc-windows-gnu --features "disable-unwind" -- -C "panic=abort" && \
    cp target/i686-pc-windows-gnu/release/examples/connect_async.exe target/i686-pc-windows-gnu/release/example_connect_async.exe && \
    wine target/i686-pc-windows-gnu/release/example_connect_async.exe
