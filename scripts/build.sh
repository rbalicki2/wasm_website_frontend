# rm -rf dist
mkdir dist
# rm -rf dist_prod
mkdir dist_prod

cp static/* dist/

echo $1
if [[ $1 = "release" ]]; then
  echo "building for release"
  cargo +nightly build --target wasm32-unknown-unknown \
    && wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_website_frontend.wasm --out-dir ./dist \
    && ../binaryen/bin/wasm-opt -Oz -o dist/wasm_website_frontend_bg.wasm dist/wasm_website_frontend_bg.wasm

else
  echo ""
  echo ""
  echo ""
  echo ""
  echo ""
  echo ""

  cargo +nightly build --target wasm32-unknown-unknown \
    && wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_website_frontend.wasm --out-dir ./dist
fi