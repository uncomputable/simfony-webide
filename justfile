# Deploy local website
serve:
    trunk serve

# Deploy local website and open in browser
open:
    trunk serve --open

# Run unit tests in WASM in Firefox
test:
    wasm-pack test --headless --firefox

# Remove all temporary files
clean:
    rm -rf target
