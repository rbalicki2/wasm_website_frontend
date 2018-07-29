PACKAGE_NAME=wasm_website_frontend
# rm -rf dist
# mkdir dist

cp static/* dist/

echo ""
echo ""
echo ""
echo ""
echo ""
echo ""

cargo +nightly build --target wasm32-unknown-unknown \
  && wasm-bindgen target/wasm32-unknown-unknown/debug/$PACKAGE_NAME.wasm --out-dir ./dist
