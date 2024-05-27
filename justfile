# Deploy local website
serve:
    RUSTFLAGS=--cfg=web_sys_unstable_apis trunk serve

# Deploy local website and open in browser
open:
    trunk serve --open

# Run unit tests in WASM in Firefox
test-wasm:
    wasm-pack test --headless --firefox

# Run unit tests on local computer
test:
    cargo test

# Run code linter
lint:
    cargo clippy --all-targets -- --deny warnings

# Format code
fmt:
    cargo fmt --all

# Check if code is formatted
fmtcheck:
    cargo fmt --all -- --check

# Check code (CI)
check:
    cargo --version
    rustc --version
    just fmtcheck
    just lint
    just test
    just test-wasm

# Remove all temporary files
clean:
    rm -rf target
