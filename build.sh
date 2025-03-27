cargo build --release || { echo "Build failed!"; exit 1; }
tar -czf NaturalTerminal-macos-v1.0.0.tar.gz -C target/release NaturalTerminal -C ../.. setup-alias.sh
echo "NaturalTerminal-macos-v1.0.0.tar.gz created with the following contents:"
tar -tzf NaturalTerminal-macos-v1.0.0.tar.gz
echo "SHA-256 checksum:"
shasum -a 256 NaturalTerminal-macos-v1.0.0.tar.gz