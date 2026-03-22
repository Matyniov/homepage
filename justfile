clippy:
    cargo clippy --fix --allow-dirty
    cargo fmt

BUNDLE_LOC:="./target/dx/maty_homepage/release/web/public"

bundle_web:
    rm -r {{BUNDLE_LOC}} || exit 0
    dx bundle --web --release --debug-symbols false --wasm-split
    cd {{BUNDLE_LOC}} && cp index.html 404.html && zip -r bundle.zip {{BUNDLE_LOC}} .
    mv "{{BUNDLE_LOC}}/bundle.zip" .

serve:
    dx serve --hot-reload true --debug-symbols true