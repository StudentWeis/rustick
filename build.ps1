cargo build --release
upx ./target/release/rustick.exe
mv ./target/release/rustick.exe ./target/release/Rustick-v0.2.2.exe