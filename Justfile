set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

test:
    cargo test --quiet

check:
    cargo fmt --all --check
    cargo clippy --all-targets --locked -- -D warnings
    cargo test --locked --quiet

# Compare gdsyntax with the Godot release it follows: `just godot-sync check`,
# or `just godot-sync update [--tag TAG]` to copy a newer one.
[windows]
godot-sync *args:
    python scripts/godot_sync.py {{args}}

[unix]
godot-sync *args:
    python3 scripts/godot_sync.py {{args}}
