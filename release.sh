cargo clean
cargo build --release
sleep 1
cp target/release/exchange .
chmod +X exchange