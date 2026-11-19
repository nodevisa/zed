#!/bin/sh
# In the example/ directory,
# - installs nodevisa-build,
# - opens it in Zed with an isolated profile (.zed-test/) which:
# - has the dev extension installed (a symlink, as Zed's "install dev extension" does),
# - installs the TOML extension,
# - trusts the worktree, so that language servers start
set -eu
cd "$PIXI_PROJECT_ROOT"

pixi install --manifest-path example/pixi.toml

mkdir -p .zed-test/extensions/installed .zed-test/config
ln -sfn "$PIXI_PROJECT_ROOT/extension" .zed-test/extensions/installed/nodevisa
cat > .zed-test/config/settings.json <<'JSON'
{
  "auto_install_extensions": { "toml": true },
  "session": { "trust_all_worktrees": true }
}
JSON

# Not the package's `zed`, which on macOS expects ~/Applications/Zed.app from a menu shortcut.
"$CONDA_PREFIX/lib/zed/zed-cli" \
  --zed "$CONDA_PREFIX/lib/zed/zed-editor" \
  --user-data-dir .zed-test \
  example example/minimal-invalid/protocol.toml
