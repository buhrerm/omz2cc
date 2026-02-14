#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");

const PLATFORMS = {
  "linux x64": "@omz2cc/linux-x64-gnu",
  "darwin x64": "@omz2cc/darwin-x64",
  "darwin arm64": "@omz2cc/darwin-arm64",
  "linux arm64": "@omz2cc/linux-arm64-gnu",
};

const key = `${process.platform} ${process.arch}`;
const pkg = PLATFORMS[key];

if (!pkg) {
  console.error(`Unsupported platform: ${process.platform} ${process.arch}`);
  process.exit(1);
}

let binPath;
try {
  binPath = path.join(require.resolve(`${pkg}/package.json`), "..", "omz2cc");
} catch {
  console.error(
    `Could not find package ${pkg}. Make sure optional dependencies were installed.`
  );
  process.exit(1);
}

try {
  execFileSync(binPath, process.argv.slice(2), {
    stdio: "inherit",
  });
} catch (e) {
  if (e.status !== null) {
    process.exit(e.status);
  }
  throw e;
}
