#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
NPM="$ROOT/npm"

# Read version from Cargo.toml
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -1)
echo "Building omz2cc v$VERSION"

# Target map: npm-dir -> rust-triple
declare -A TARGETS=(
  ["linux-x64-gnu"]="x86_64-unknown-linux-gnu"
  ["linux-arm64-gnu"]="aarch64-unknown-linux-gnu"
  ["darwin-x64"]="x86_64-apple-darwin"
  ["darwin-arm64"]="aarch64-apple-darwin"
)

# Sync versions across all package.json files
echo "Syncing version $VERSION to package.json files..."
sed -i'' -e "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$NPM/omz2cc/package.json"
for dir in "${!TARGETS[@]}"; do
  sed -i'' -e "s|\"@omz2cc/$dir\": \"[^\"]*\"|\"@omz2cc/$dir\": \"$VERSION\"|" "$NPM/omz2cc/package.json"
  sed -i'' -e "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$NPM/$dir/package.json"
done

# Detect native target
NATIVE_ARCH="$(uname -m)"
NATIVE_OS="$(uname -s)"
NATIVE_TRIPLE=""
if [[ "$NATIVE_OS" == "Linux" && "$NATIVE_ARCH" == "x86_64" ]]; then
  NATIVE_TRIPLE="x86_64-unknown-linux-gnu"
elif [[ "$NATIVE_OS" == "Linux" && "$NATIVE_ARCH" == "aarch64" ]]; then
  NATIVE_TRIPLE="aarch64-unknown-linux-gnu"
elif [[ "$NATIVE_OS" == "Darwin" && "$NATIVE_ARCH" == "x86_64" ]]; then
  NATIVE_TRIPLE="x86_64-apple-darwin"
elif [[ "$NATIVE_OS" == "Darwin" && "$NATIVE_ARCH" == "arm64" ]]; then
  NATIVE_TRIPLE="aarch64-apple-darwin"
fi

# Ensure required tools for cross-compilation
ensure_target() {
  local triple="$1"
  if ! rustup target list --installed | grep -q "$triple"; then
    echo "  Installing rustup target $triple..."
    rustup target add "$triple"
  fi
}

# Build each target
BUILD_ONLY="${BUILD_ONLY:-}"
for dir in "${!TARGETS[@]}"; do
  triple="${TARGETS[$dir]}"

  if [ -n "$BUILD_ONLY" ] && [ "$BUILD_ONLY" != "$dir" ]; then
    echo "Skipping $dir (BUILD_ONLY=$BUILD_ONLY)"
    continue
  fi

  echo "Building for $triple..."

  if [[ "$triple" == "$NATIVE_TRIPLE" ]]; then
    # Native target — no cross-compilation needed
    cargo build --release --target "$triple"
  elif command -v cross &>/dev/null && docker info &>/dev/null 2>&1; then
    # Cross-compile via cross+Docker
    cross build --release --target "$triple"
  else
    # Try cargo with rustup target (works for same-OS different-arch)
    ensure_target "$triple"
    cargo build --release --target "$triple"
  fi

  # Copy binary
  cp "$ROOT/target/$triple/release/omz2cc" "$NPM/$dir/omz2cc"
  chmod +x "$NPM/$dir/omz2cc"
  echo "  -> npm/$dir/omz2cc"
done

echo "Done. Packages ready in npm/"
