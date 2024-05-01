TARGET_DIR=~/.local/bin/

cargo build --release
mkdir -p "$TARGET_DIR"
cp target/release/rex_ai "$TARGET_DIR/rex_ai"
echo "Ensure '\$OPENAI_API_KEY' is set in your environment"
echo "Configure model using '\$REX_AI_MODEL' (default: 'gpt-3.5-turbo')"
echo "Add to .bashrc or .zshrc: export PATH=\$PATH:$TARGET_DIR"
echo "Call using 'rex_ai -- \"say hi!\""