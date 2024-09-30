cargo build --release
upx -9 ./target/release/rustick.exe
mv ./target/release/rustick.exe ./target/release/Rustick-v0.2.3.exe