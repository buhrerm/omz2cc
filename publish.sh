#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

# Get version from Cargo.toml
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
echo "Publishing omz2cc v$VERSION"
echo

# Sync all package.json versions
for dir in linux-x64-gnu linux-arm64-gnu darwin-x64 darwin-arm64; do
  sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/$dir/package.json
done
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/omz2cc/package.json
for dir in linux-x64-gnu linux-arm64-gnu darwin-x64 darwin-arm64; do
  sed -i "s|\"@omz2cc/$dir\": \"[^\"]*\"|\"@omz2cc/$dir\": \"$VERSION\"|" npm/omz2cc/package.json
done
echo "Synced all package.json versions to $VERSION"

# Build all targets
echo
echo "Building linux-x64 (native)..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "Building linux-arm64 (cross)..."
cross build --release --target aarch64-unknown-linux-gnu

echo "Building darwin-x64 (zigbuild)..."
PATH="/tmp/zig-linux-x86_64-0.13.0:$PATH" cargo zigbuild --release --target x86_64-apple-darwin

echo "Building darwin-arm64 (zigbuild)..."
PATH="/tmp/zig-linux-x86_64-0.13.0:$PATH" cargo zigbuild --release --target aarch64-apple-darwin

# Copy binaries
cp target/x86_64-unknown-linux-gnu/release/omz2cc  npm/linux-x64-gnu/omz2cc
cp target/aarch64-unknown-linux-gnu/release/omz2cc npm/linux-arm64-gnu/omz2cc
cp target/x86_64-apple-darwin/release/omz2cc       npm/darwin-x64/omz2cc
cp target/aarch64-apple-darwin/release/omz2cc      npm/darwin-arm64/omz2cc
chmod +x npm/*/omz2cc
echo
echo "All 4 binaries built and copied"

# Publish
echo
for dir in linux-x64-gnu linux-arm64-gnu darwin-x64 darwin-arm64 omz2cc; do
  echo "Publishing $(jq -r .name npm/$dir/package.json)@$VERSION..."
  (cd npm/$dir && npm publish --access public)
done

echo
echo "Done! Published omz2cc@$VERSION"
