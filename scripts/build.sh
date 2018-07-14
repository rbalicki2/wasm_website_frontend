PACKAGE_NAME=wasm_website_frontend
# rm -rf dist
# mkdir dist

cp static/* dist/

cargo +nightly build --verbose --target wasm32-unknown-unknown \
  && wasm-bindgen target/wasm32-unknown-unknown/debug/$PACKAGE_NAME.wasm --out-dir ./dist
