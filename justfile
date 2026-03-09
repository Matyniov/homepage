clippy:
    cargo clippy --fix --allow-dirty
    cargo fmt

BUNDLE_LOC:="./target/dx/maty_homepage/release/web"

# https://rustwasm.github.io/docs/book/reference/code-size.html
bundle_web:
    rm -r {{BUNDLE_LOC}}
    dx bundle --web --release --debug-symbols false --wasm-split

serve:
    dx serve --hot-reload true --debug-symbols true