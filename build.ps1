cargo build --release
upx ./target/release/rustick.exe
mv ./target/release/rustick.exe ./target/release/Rustick-v0.2.1.exe