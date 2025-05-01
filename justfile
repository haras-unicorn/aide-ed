set windows-shell := ["nu.exe", "-c"]
set shell := ["nu", "-c"]

root := absolute_path('')

default:
    @just --choose

dev *args:
    dx serve --package aide-ed-server --port 8081

test:
    cd '{{ root }}'; cargo test

format:
    cd '{{ root }}'; just --unstable --fmt
    prettier --write '{{ root }}'
    nixpkgs-fmt '{{ root }}'
    cd '{{ root }}'; cargo fmt --all

lint:
    cd '{{ root }}'; just --unstable --fmt --check
    prettier --check '{{ root }}'
    cspell lint '{{ root }}' --no-progress
    nixpkgs-fmt '{{ root }}' --check
    markdownlint --ignore-path .gitignore '{{ root }}'
    markdown-link-check \
      --config .markdown-link-check.json \
      --quiet \
      ...(fd '.*.md' | lines)
    cd '{{ root }}'; \
      $env.DATABASE_URL = null; \
      cargo clippy -- -D warnings

upgrade:
    nix flake update
    cargo upgrade

docs:
    rm -rf '{{ root }}/artifacts'
    mkdir '{{ root }}/artifacts'
    cd '{{ root }}'; $env.DATABASE_URL = null; cargo doc --no-deps
    cd '{{ root }}/docs/user/en'; mdbook build
    cd '{{ root }}/docs/user/hr'; mdbook build
    cd '{{ root }}/docs/dev'; mdbook build
    mv '{{ root }}/target/doc' '{{ root }}/artifacts/code'
    mv '{{ root }}/docs/user/en/book' '{{ root }}/artifacts/user/en'
    mv '{{ root }}/docs/user/hr/book' '{{ root }}/artifacts/user/hr'
    mv '{{ root }}/docs/dev/book' '{{ root }}/artifacts/dev'
    cp '{{ root }}/docs/index.html' '{{ root }}/artifacts'

raspberryPi4 *args:
    {{ root }}/scripts/flake/raspberryPi4.nu {{ args }}
