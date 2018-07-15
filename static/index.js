const wasm = import("./wasm_website_frontend");

wasm.then(module => {
  module.start();
});
