TARGET_DIR=~/.local/bin/

cargo build --release
mkdir -p "$TARGET_DIR"
cp target/release/rex_ai "$TARGET_DIR/rex_ai"

